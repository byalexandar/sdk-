import { CanisterId, GlobalInternetComputer, HttpAgent, IDL } from '@dfinity/agent';
import { createAgent, SiteInfo } from './host';

declare const window: GlobalInternetComputer & Window;

// Retrieve and execute a JavaScript file from the server.
async function _loadJs(
  canisterId: CanisterId,
  filename: string,
  onload = async () => {},
): Promise<any> {
  const idlFn = ({ IDL: idl }: any) => {
    return idl.Service({
      retrieve: idl.Func([idl.Text], [idl.Vec(idl.Nat8)], ['query']),
    });
  };

  const actor = window.ic.agent.makeActorFactory(idlFn)({
    canisterId,
  });

  const content = (await actor.retrieve(filename)) as number[];
  const js = new TextDecoder().decode(new Uint8Array(content));
  // const dataUri = new Function(js);

  // Run an event function so the callee can execute some code before loading the
  // Javascript.
  await onload();

  // TODO(hansl): either get rid of eval, or rid of webpack, or make this
  // work without this horrible hack.
  return eval(js); // tslint:disable-line
}

async function _loadCandid(canisterId: CanisterId): Promise<any> {
  const origin = window.location.origin;
  const url = `${origin}/_/candid?canisterId=${canisterId.toText()}&format=js`;
  const response = await fetch(url);
  if (!response.ok) {
    throw new Error(`Cannot fetch candid file`);
  }
  const js = await response.text();
  const dataUri = 'data:text/javascript;charset=utf-8,' + encodeURIComponent(js);
  // TODO(hansl): either get rid of eval, or rid of webpack, or make this
  // work without this horrible hack.
  return eval('import("' + dataUri + '")'); // tslint:disable-line
}

async function _main() {
  const site = await SiteInfo.fromWindow();
  const agent = await createAgent(site);
  window.ic = { agent, HttpAgent, IDL };

  // Find the canister ID. Allow override from the url with 'canister_id=1234..'
  const canisterId = site.canisterId;
  if (!canisterId) {
    // Show an error.
    const div = document.createElement('div');
    div.innerText =
      'Could not find the canister ID to use. Please provide one in the query parameters.';

    document.body.replaceChild(div, document.body.getElementsByTagName('app').item(0)!);
  } else {
    if (window.location.pathname === '/candid') {
      // Load candid.did.js from endpoint.
      const candid = await _loadCandid(canisterId);
      const canister = window.ic.agent.makeActorFactory(candid.default)({ canisterId });
      // @ts-ignore: Could not find a declaration file for module
      const render: any = await import('./candid/candid.js');
      render.render(canisterId, canister);
    } else {
      // Load index.js from the canister and execute it.
      await _loadJs(canisterId, 'index.js', async () => {
        document.getElementById('ic-progress')!.remove();
      });
    }
  }
}

_main().catch(err => {
  const div = document.createElement('div');
  div.innerText = 'An error happened:';
  const pre = document.createElement('pre');
  pre.innerHTML = err.stack;
  div.appendChild(pre);
  document.body.replaceChild(div, document.body.getElementsByTagName('app').item(0)!);
  throw err;
});