let P=0,S=`string`,Q=1,T=`Object`,N=`utf-8`,L=null,M=`undefined`,U=4,R=`function`,J=128,I=Array,O=Error,V=Object,W=Reflect,K=undefined;var C=(async(a,b)=>{if(typeof Response===R&&a instanceof Response){if(typeof WebAssembly.instantiateStreaming===R){try{return await WebAssembly.instantiateStreaming(a,b)}catch(b){if(a.headers.get(`Content-Type`)!=`application/wasm`){console.warn(`\`WebAssembly.instantiateStreaming\` failed because your server does not serve wasm with \`application/wasm\` MIME type. Falling back to \`WebAssembly.instantiate\` which is slower. Original error:\\n`,b)}else{throw b}}};const c=await a.arrayBuffer();return await WebAssembly.instantiate(c,b)}else{const c=await WebAssembly.instantiate(a,b);if(c instanceof WebAssembly.Instance){return {instance:c,module:a}}else{return c}}});var s=(a=>{const b=typeof a;if(b==`number`||b==`boolean`||a==L){return `${a}`};if(b==S){return `"${a}"`};if(b==`symbol`){const b=a.description;if(b==L){return `Symbol`}else{return `Symbol(${b})`}};if(b==R){const b=a.name;if(typeof b==S&&b.length>P){return `Function(${b})`}else{return `Function`}};if(I.isArray(a)){const b=a.length;let c=`[`;if(b>P){c+=s(a[P])};for(let d=Q;d<b;d++){c+=`, `+ s(a[d])};c+=`]`;return c};const c=/\[object ([^\]]+)\]/.exec(toString.call(a));let d;if(c.length>Q){d=c[Q]}else{return toString.call(a)};if(d==T){try{return `Object(`+ JSON.stringify(a)+ `)`}catch(a){return T}};if(a instanceof O){return `${a.name}: ${a.message}\n${a.stack}`};return d});var E=((a,b)=>{});var A=((a,b)=>{a=a>>>P;const c=z();const d=c.subarray(a/U,a/U+ b);const e=[];for(let a=P;a<d.length;a++){e.push(k(d[a]))};return e});var k=(a=>{const b=c(a);j(a);return b});var i=(a=>{if(h===b.length)b.push(b.length+ Q);const c=h;h=b[c];b[c]=a;return c});var w=((c,d,e)=>{try{a.wasm_bindgen__convert__closures__invoke1_mut_ref__h88c61e6755580a29(c,d,v(e))}finally{b[u++]=K}});function B(b,c){try{return b.apply(this,c)}catch(b){a.__wbindgen_exn_store(i(b))}}var H=(async(b)=>{if(a!==K)return a;if(typeof b===M){b=new URL(`ideal-wages-calculator-f7771da8876f74805119daff72e5ba0360ba792b00ab3ec5cffdd005111e3bbd7f1159705aef9b6efd592b31ac4934ed_bg.wasm`,import.meta.url)};const c=D();if(typeof b===S||typeof Request===R&&b instanceof Request||typeof URL===R&&b instanceof URL){b=fetch(b)};E(c);const {instance:d,module:e}=await C(await b,c);return F(d,e)});var r=(()=>{if(q===L||q.byteLength===P){q=new Int32Array(a.memory.buffer)};return q});var p=(a=>a===K||a===L);var c=(a=>b[a]);var G=(b=>{if(a!==K)return a;const c=D();E(c);if(!(b instanceof WebAssembly.Module)){b=new WebAssembly.Module(b)};const d=new WebAssembly.Instance(b,c);return F(d,b)});var F=((b,c)=>{a=b.exports;H.__wbindgen_wasm_module=c;q=L;y=L;e=L;a.__wbindgen_start();return a});var D=(()=>{const b={};b.wbg={};b.wbg.__wbg_instanceof_HtmlInputElement_31b50e0cf542c524=(a=>{let b;try{b=c(a) instanceof HTMLInputElement}catch{b=!1}const d=b;return d});b.wbg.__wbindgen_is_undefined=(a=>{const b=c(a)===K;return b});b.wbg.__wbg_newwithstr_3d9bc779603a93c7=function(){return B(((a,b)=>{const c=new Request(g(a,b));return i(c)}),arguments)};b.wbg.__wbg_url_fda63503ced387ff=((b,d)=>{const e=c(d).url;const f=o(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=l;r()[b/U+ Q]=g;r()[b/U+ P]=f});b.wbg.__wbg_new_a76f6bcb38f791ea=function(){return B(((a,b)=>{const c=new URL(g(a,b));return i(c)}),arguments)};b.wbg.__wbg_setsearch_16b87f04ea0e6b80=((a,b,d)=>{c(a).search=g(b,d)});b.wbg.__wbg_toString_a8e343996af880e9=(a=>{const b=c(a).toString();return i(b)});b.wbg.__wbindgen_string_new=((a,b)=>{const c=g(a,b);return i(c)});b.wbg.__wbindgen_object_clone_ref=(a=>{const b=c(a);return i(b)});b.wbg.__wbg_newwithstrandinit_cad5cd6038c7ff5d=function(){return B(((a,b,d)=>{const e=new Request(g(a,b),c(d));return i(e)}),arguments)};b.wbg.__wbg_fetch_336b6f0cb426b46e=((a,b)=>{const d=c(a).fetch(c(b));return i(d)});b.wbg.__wbg_instanceof_WorkerGlobalScope_d9d741da0fb130ce=(a=>{let b;try{b=c(a) instanceof WorkerGlobalScope}catch{b=!1}const d=b;return d});b.wbg.__wbg_fetch_8eaf01857a5bb21f=((a,b)=>{const d=c(a).fetch(c(b));return i(d)});b.wbg.__wbg_instanceof_Response_fc4327dbfcdf5ced=(a=>{let b;try{b=c(a) instanceof Response}catch{b=!1}const d=b;return d});b.wbg.__wbg_text_a667ac1770538491=function(){return B((a=>{const b=c(a).text();return i(b)}),arguments)};b.wbg.__wbg_body_674aec4c1c0910cd=(a=>{const b=c(a).body;return p(b)?P:i(b)});b.wbg.__wbg_lastChild_0cee692010bac6c2=(a=>{const b=c(a).lastChild;return p(b)?P:i(b)});b.wbg.__wbg_error_788ae33f81d3b84b=(a=>{console.error(c(a))});b.wbg.__wbg_instanceof_HtmlSelectElement_75d8a9ac3b088f08=(a=>{let b;try{b=c(a) instanceof HTMLSelectElement}catch{b=!1}const d=b;return d});b.wbg.__wbg_new_abda76e883ba8a5f=(()=>{const a=new O();return i(a)});b.wbg.__wbg_stack_658279fe44541cf6=((b,d)=>{const e=c(d).stack;const f=o(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=l;r()[b/U+ Q]=g;r()[b/U+ P]=f});b.wbg.__wbg_error_f851667af71bcfc6=((b,c)=>{let d;let e;try{d=b;e=c;console.error(g(b,c))}finally{a.__wbindgen_free(d,e,Q)}});b.wbg.__wbindgen_object_drop_ref=(a=>{k(a)});b.wbg.__wbg_instanceof_Error_ab19e20608ea43c7=(a=>{let b;try{b=c(a) instanceof O}catch{b=!1}const d=b;return d});b.wbg.__wbg_name_8f734cbbd6194153=(a=>{const b=c(a).name;return i(b)});b.wbg.__wbg_message_48bacc5ea57d74ee=(a=>{const b=c(a).message;return i(b)});b.wbg.__wbg_toString_1c056108b87ba68b=(a=>{const b=c(a).toString();return i(b)});b.wbg.__wbg_new_b51585de1b234aff=(()=>{const a=new V();return i(a)});b.wbg.__wbg_new_1eead62f64ca15ce=function(){return B((()=>{const a=new Headers();return i(a)}),arguments)};b.wbg.__wbg_new_2a98b9c4a51bdc04=function(){return B((()=>{const a=new URLSearchParams();return i(a)}),arguments)};b.wbg.__wbg_document_f7ace2b956f30a4f=(a=>{const b=c(a).document;return p(b)?P:i(b)});b.wbg.__wbg_get_97b561fb56f034b5=function(){return B(((a,b)=>{const d=W.get(c(a),c(b));return i(d)}),arguments)};b.wbg.__wbindgen_string_get=((b,d)=>{const e=c(d);const f=typeof e===S?e:K;var g=p(f)?P:o(f,a.__wbindgen_malloc,a.__wbindgen_realloc);var h=l;r()[b/U+ Q]=h;r()[b/U+ P]=g});b.wbg.__wbg_self_1ff1d729e9aae938=function(){return B((()=>{const a=self.self;return i(a)}),arguments)};b.wbg.__wbg_window_5f4faef6c12b79ec=function(){return B((()=>{const a=window.window;return i(a)}),arguments)};b.wbg.__wbg_globalThis_1d39714405582d3c=function(){return B((()=>{const a=globalThis.globalThis;return i(a)}),arguments)};b.wbg.__wbg_global_651f05c6a0944d1c=function(){return B((()=>{const a=global.global;return i(a)}),arguments)};b.wbg.__wbg_newnoargs_581967eacc0e2604=((a,b)=>{const c=new Function(g(a,b));return i(c)});b.wbg.__wbg_call_cb65541d95d71282=function(){return B(((a,b)=>{const d=c(a).call(c(b));return i(d)}),arguments)};b.wbg.__wbg_is_205d914af04a8faa=((a,b)=>{const d=V.is(c(a),c(b));return d});b.wbg.__wbg_set_092e06b0f9d71865=function(){return B(((a,b,d)=>{const e=W.set(c(a),c(b),c(d));return e}),arguments)};b.wbg.__wbindgen_debug_string=((b,d)=>{const e=s(c(d));const f=o(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=l;r()[b/U+ Q]=g;r()[b/U+ P]=f});b.wbg.__wbindgen_throw=((a,b)=>{throw new O(g(a,b))});b.wbg.__wbindgen_cb_drop=(a=>{const b=k(a).original;if(b.cnt--==Q){b.a=P;return !0};const c=!1;return c});b.wbg.__wbg_then_f7e06ee3c11698eb=((a,b)=>{const d=c(a).then(c(b));return i(d)});b.wbg.__wbg_then_b2267541e2a73865=((a,b,d)=>{const e=c(a).then(c(b),c(d));return i(e)});b.wbg.__wbg_resolve_53698b95aaf7fcf8=(a=>{const b=Promise.resolve(c(a));return i(b)});b.wbg.__wbg_instanceof_Window_9029196b662bc42a=(a=>{let b;try{b=c(a) instanceof Window}catch{b=!1}const d=b;return d});b.wbg.__wbg_createElement_4891554b28d3388b=function(){return B(((a,b,d)=>{const e=c(a).createElement(g(b,d));return i(e)}),arguments)};b.wbg.__wbg_instanceof_Element_4622f5da1249a3eb=(a=>{let b;try{b=c(a) instanceof Element}catch{b=!1}const d=b;return d});b.wbg.__wbg_target_f171e89c61e2bccf=(a=>{const b=c(a).target;return p(b)?P:i(b)});b.wbg.__wbg_setchecked_e5a50baea447b8a8=((a,b)=>{c(a).checked=b!==P});b.wbg.__wbg_value_9423da9d988ee8cf=((b,d)=>{const e=c(d).value;const f=o(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=l;r()[b/U+ Q]=g;r()[b/U+ P]=f});b.wbg.__wbg_setvalue_1f95e61cbc382f7f=((a,b,d)=>{c(a).value=g(b,d)});b.wbg.__wbg_value_c45528fab757534f=((b,d)=>{const e=c(d).value;const f=o(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=l;r()[b/U+ Q]=g;r()[b/U+ P]=f});b.wbg.__wbg_setvalue_0dc100d4b9908028=((a,b,d)=>{c(a).value=g(b,d)});b.wbg.__wbg_nextSibling_304d9aac7c2774ae=(a=>{const b=c(a).nextSibling;return p(b)?P:i(b)});b.wbg.__wbg_insertBefore_ffa01d4b747c95fc=function(){return B(((a,b,d)=>{const e=c(a).insertBefore(c(b),c(d));return i(e)}),arguments)};b.wbg.__wbg_removeChild_973429f368206138=function(){return B(((a,b)=>{const d=c(a).removeChild(c(b));return i(d)}),arguments)};b.wbg.__wbg_search_2ff3bb9114e0ca34=((b,d)=>{const e=c(d).search;const f=o(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=l;r()[b/U+ Q]=g;r()[b/U+ P]=f});b.wbg.__wbg_addEventListener_a5963e26cd7b176b=function(){return B(((a,b,d,e,f)=>{c(a).addEventListener(g(b,d),c(e),c(f))}),arguments)};b.wbg.__wbg_composedPath_cf1bb5b8bcff496f=(a=>{const b=c(a).composedPath();return i(b)});b.wbg.__wbg_length_fff51ee6522a1a18=(a=>{const b=c(a).length;return b});b.wbg.__wbg_cachekey_b61393159c57fd7b=((a,b)=>{const d=c(b).__yew_subtree_cache_key;r()[a/U+ Q]=p(d)?P:d;r()[a/U+ P]=!p(d)});b.wbg.__wbg_subtreeid_e348577f7ef777e3=((a,b)=>{const d=c(b).__yew_subtree_id;r()[a/U+ Q]=p(d)?P:d;r()[a/U+ P]=!p(d)});b.wbg.__wbg_get_44be0491f933a435=((a,b)=>{const d=c(a)[b>>>P];return i(d)});b.wbg.__wbg_bubbles_63572b91f3885ef1=(a=>{const b=c(a).bubbles;return b});b.wbg.__wbg_setsubtreeid_d32e6327eef1f7fc=((a,b)=>{c(a).__yew_subtree_id=b>>>P});b.wbg.__wbg_setcachekey_80183b7cfc421143=((a,b)=>{c(a).__yew_subtree_cache_key=b>>>P});b.wbg.__wbg_cancelBubble_90d1c3aa2a76cbeb=(a=>{const b=c(a).cancelBubble;return b});b.wbg.__wbg_listenerid_12315eee21527820=((a,b)=>{const d=c(b).__yew_listener_id;r()[a/U+ Q]=p(d)?P:d;r()[a/U+ P]=!p(d)});b.wbg.__wbg_parentElement_c75962bc9997ea5f=(a=>{const b=c(a).parentElement;return p(b)?P:i(b)});b.wbg.__wbg_parentNode_9e53f8b17eb98c9d=(a=>{const b=c(a).parentNode;return p(b)?P:i(b)});b.wbg.__wbg_instanceof_ShadowRoot_b64337370f59fe2d=(a=>{let b;try{b=c(a) instanceof ShadowRoot}catch{b=!1}const d=b;return d});b.wbg.__wbg_host_e1c47c33975060d3=(a=>{const b=c(a).host;return i(b)});b.wbg.__wbg_setnodeValue_d1c8382910b45e04=((a,b,d)=>{c(a).nodeValue=b===P?K:g(b,d)});b.wbg.__wbg_value_3c5f08ffc2b7d6f9=((b,d)=>{const e=c(d).value;const f=o(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=l;r()[b/U+ Q]=g;r()[b/U+ P]=f});b.wbg.__wbg_namespaceURI_31718ed49b5343a3=((b,d)=>{const e=c(d).namespaceURI;var f=p(e)?P:o(e,a.__wbindgen_malloc,a.__wbindgen_realloc);var g=l;r()[b/U+ Q]=g;r()[b/U+ P]=f});b.wbg.__wbg_createElementNS_119acf9e82482041=function(){return B(((a,b,d,e,f)=>{const h=c(a).createElementNS(b===P?K:g(b,d),g(e,f));return i(h)}),arguments)};b.wbg.__wbg_createTextNode_2fd22cd7e543f938=((a,b,d)=>{const e=c(a).createTextNode(g(b,d));return i(e)});b.wbg.__wbg_appendChild_51339d4cde00ee22=function(){return B(((a,b)=>{const d=c(a).appendChild(c(b));return i(d)}),arguments)};b.wbg.__wbg_error_71d6845bf00a930f=((b,c)=>{var d=A(b,c).slice();a.__wbindgen_free(b,c*U);console.error(...d)});b.wbg.__wbg_setinnerHTML_b089587252408b67=((a,b,d)=>{c(a).innerHTML=g(b,d)});b.wbg.__wbg_children_27ed308801b57d3f=(a=>{const b=c(a).children;return i(b)});b.wbg.__wbg_from_d7c216d4616bb368=(a=>{const b=I.from(c(a));return i(b)});b.wbg.__wbg_setlistenerid_3183aae8fa5840fb=((a,b)=>{c(a).__yew_listener_id=b>>>P});b.wbg.__wbg_setAttribute_e7e80b478b7b8b2f=function(){return B(((a,b,d,e,f)=>{c(a).setAttribute(g(b,d),g(e,f))}),arguments)};b.wbg.__wbg_removeAttribute_d8404da431968808=function(){return B(((a,b,d)=>{c(a).removeAttribute(g(b,d))}),arguments)};b.wbg.__wbindgen_closure_wrapper640=((a,b,c)=>{const d=t(a,b,35,w);return i(d)});b.wbg.__wbindgen_closure_wrapper1333=((a,b,c)=>{const d=t(a,b,50,x);return i(d)});return b});var z=(()=>{if(y===L||y.byteLength===P){y=new Uint32Array(a.memory.buffer)};return y});var x=((b,c,d)=>{a.wasm_bindgen__convert__closures__invoke1_mut__hed82626e0e840f0a(b,c,i(d))});var j=(a=>{if(a<132)return;b[a]=h;h=a});var t=((b,c,d,e)=>{const f={a:b,b:c,cnt:Q,dtor:d};const g=(...b)=>{f.cnt++;const c=f.a;f.a=P;try{return e(c,f.b,...b)}finally{if(--f.cnt===P){a.__wbindgen_export_2.get(f.dtor)(c,f.b)}else{f.a=c}}};g.original=f;return g});var o=((a,b,c)=>{if(c===K){const c=m.encode(a);const d=b(c.length,Q)>>>P;f().subarray(d,d+ c.length).set(c);l=c.length;return d};let d=a.length;let e=b(d,Q)>>>P;const g=f();let h=P;for(;h<d;h++){const b=a.charCodeAt(h);if(b>127)break;g[e+ h]=b};if(h!==d){if(h!==P){a=a.slice(h)};e=c(e,d,d=h+ a.length*3,Q)>>>P;const b=f().subarray(e+ h,e+ d);const g=n(a,b);h+=g.written};l=h;return e});var f=(()=>{if(e===L||e.byteLength===P){e=new Uint8Array(a.memory.buffer)};return e});var g=((a,b)=>{a=a>>>P;return d.decode(f().subarray(a,a+ b))});var v=(a=>{if(u==Q)throw new O(`out of js stack`);b[--u]=a;return u});let a;const b=new I(J).fill(K);b.push(K,L,!0,!1);const d=typeof TextDecoder!==M?new TextDecoder(N,{ignoreBOM:!0,fatal:!0}):{decode:()=>{throw O(`TextDecoder not available`)}};if(typeof TextDecoder!==M){d.decode()};let e=L;let h=b.length;let l=P;const m=typeof TextEncoder!==M?new TextEncoder(N):{encode:()=>{throw O(`TextEncoder not available`)}};const n=typeof m.encodeInto===R?((a,b)=>m.encodeInto(a,b)):((a,b)=>{const c=m.encode(a);b.set(c);return {read:a.length,written:c.length}});let q=L;let u=J;let y=L;export default H;export{G as initSync}