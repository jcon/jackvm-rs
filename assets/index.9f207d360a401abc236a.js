(()=>{"use strict";var e,n,t,r,o,a={160:(e,n,t)=>{t.a(e,(async e=>{t.d(n,{io:()=>o.io});var r=t(314),o=t(304),a=e([o,r]);[o,r]=a.then?await a:a}))},304:(e,n,t)=>{t.a(e,(async r=>{t.d(n,{io:()=>C,ug:()=>L,G6:()=>T,h4:()=>I,L7:()=>P,m_:()=>F,h9:()=>H,Dz:()=>M,kF:()=>$,wp:()=>D,NI:()=>G,Cf:()=>q,AT:()=>R,Vl:()=>W,S0:()=>J,SE:()=>U,wl:()=>B,jm:()=>K,GJ:()=>N,Gr:()=>X,O8:()=>V,jL:()=>z,Rs:()=>Q,XJ:()=>Y,HQ:()=>Z,Mw:()=>ee,nM:()=>ne,$2:()=>te,I1:()=>re,Ic:()=>oe,dr:()=>ae,Zt:()=>ce,XP:()=>_e,M1:()=>ie,fY:()=>ue,Or:()=>le,G4:()=>se,rh:()=>fe});var o=t(314);e=t.hmd(e);var a=r([o]);o=(a.then?await a:a)[0];const c=new Array(32).fill(void 0);function _(e){return c[e]}c.push(void 0,null,!0,!1);let i=c.length;function u(e){const n=_(e);return function(e){e<36||(c[e]=i,i=e)}(e),n}let l=new("undefined"==typeof TextDecoder?(0,e.require)("util").TextDecoder:TextDecoder)("utf-8",{ignoreBOM:!0,fatal:!0});l.decode();let s=null;function f(){return null!==s&&s.buffer===o.memory.buffer||(s=new Uint8Array(o.memory.buffer)),s}function d(e,n){return l.decode(f().subarray(e,e+n))}function b(e){i===c.length&&c.push(c.length+1);const n=i;return i=c[n],c[n]=e,n}function p(e){return null==e}let g=null,w=null;function m(){return null!==w&&w.buffer===o.memory.buffer||(w=new Int32Array(o.memory.buffer)),w}function h(e){const n=typeof e;if("number"==n||"boolean"==n||null==e)return`${e}`;if("string"==n)return`"${e}"`;if("symbol"==n){const n=e.description;return null==n?"Symbol":`Symbol(${n})`}if("function"==n){const n=e.name;return"string"==typeof n&&n.length>0?`Function(${n})`:"Function"}if(Array.isArray(e)){const n=e.length;let t="[";n>0&&(t+=h(e[0]));for(let r=1;r<n;r++)t+=", "+h(e[r]);return t+="]",t}const t=/\[object ([^\]]+)\]/.exec(toString.call(e));let r;if(!(t.length>1))return toString.call(e);if(r=t[1],"Object"==r)try{return"Object("+JSON.stringify(e)+")"}catch(e){return"Object"}return e instanceof Error?`${e.name}: ${e.message}\n${e.stack}`:r}let y=0,v=new("undefined"==typeof TextEncoder?(0,e.require)("util").TextEncoder:TextEncoder)("utf-8");const j="function"==typeof v.encodeInto?function(e,n){return v.encodeInto(e,n)}:function(e,n){const t=v.encode(e);return n.set(t),{read:e.length,written:t.length}};function x(e,n,t){if(void 0===t){const t=v.encode(e),r=n(t.length);return f().subarray(r,r+t.length).set(t),y=t.length,r}let r=e.length,o=n(r);const a=f();let c=0;for(;c<r;c++){const n=e.charCodeAt(c);if(n>127)break;a[o+c]=n}if(c!==r){0!==c&&(e=e.slice(c)),o=t(o,r,r=c+3*e.length);const n=f().subarray(o+c,o+r);c+=j(e,n).written}return y=c,o}function k(e,n,t,r){const a={a:e,b:n,cnt:1,dtor:t},c=(...e)=>{a.cnt++;const n=a.a;a.a=0;try{return r(n,a.b,...e)}finally{0==--a.cnt?o.__wbindgen_export_2.get(a.dtor)(n,a.b):a.a=n}};return c.original=a,c}function E(e,n){o._dyn_core__ops__function__FnMut_____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h7fc2d71d91357a99(e,n)}function S(e,n,t){o._dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h4f43b457ecf8c443(e,n,b(t))}function O(e){return function(){try{return e.apply(this,arguments)}catch(e){o.__wbindgen_exn_store(b(e))}}}let A=null;class C{static __wrap(e){const n=Object.create(C.prototype);return n.ptr=e,n}free(){const e=this.ptr;this.ptr=0,o.__wbg_jackvmplayer_free(e)}constructor(e){var n=o.jackvmplayer_new(b(e));return C.__wrap(n)}load(e){var n=x(e,o.__wbindgen_malloc,o.__wbindgen_realloc),t=y;o.jackvmplayer_load(this.ptr,n,t)}addHaltListener(e){o.jackvmplayer_addHaltListener(this.ptr,b(e))}handleHalt(){o.jackvmplayer_handleHalt(this.ptr)}isHalted(){return 0!==o.jackvmplayer_isHalted(this.ptr)}pause(){o.jackvmplayer_pause(this.ptr)}isStopped(){return 0!==o.jackvmplayer_isStopped(this.ptr)}isPaused(){return 0!==o.jackvmplayer_isPaused(this.ptr)}setIsPaused(e){o.jackvmplayer_setIsPaused(this.ptr,e)}restart(){o.jackvmplayer_restart(this.ptr)}copyScreen(){o.jackvmplayer_copyScreen(this.ptr)}nextFrame(){o.jackvmplayer_nextFrame(this.ptr)}run(){o.jackvmplayer_run(this.ptr)}handleKeyDown(e){o.jackvmplayer_handleKeyDown(this.ptr,b(e))}handleKeyUp(){o.jackvmplayer_handleKeyUp(this.ptr)}}const L=function(e){u(e)},T=function(e){const n=u(e).original;return 1==n.cnt--&&(n.a=0,!0)},I=function(e,n){return b(d(e,n))},P=function(e,n){alert(d(e,n))},F=function(e){return b(_(e))},H=function(){return b(new Error)},M=function(e,n){var t=x(_(n).stack,o.__wbindgen_malloc,o.__wbindgen_realloc),r=y;m()[e/4+1]=r,m()[e/4+0]=t},$=function(e,n){try{console.error(d(e,n))}finally{o.__wbindgen_free(e,n)}},D=function(e){return _(e)instanceof Window},G=function(e){var n=_(e).document;return p(n)?0:b(n)},q=O((function(e,n){return _(e).requestAnimationFrame(_(n))})),R=function(e){var n=_(e).body;return p(n)?0:b(n)},W=O((function(e,n,t){return b(_(e).createElement(d(n,t)))})),J=O((function(e,n,t,r){_(e).addEventListener(d(n,t),_(r))})),U=O((function(e,n,t){return b(new ImageData((r=e,a=n,(null!==A&&A.buffer===o.memory.buffer||(A=new Uint8ClampedArray(o.memory.buffer)),A).subarray(r/1,r/1+a)),t>>>0));var r,a})),B=function(e){console.log(_(e))},K=function(e){return _(e)instanceof HTMLElement},N=function(e){return _(e)instanceof CanvasRenderingContext2D},X=O((function(e,n,t,r){_(e).putImageData(_(n),t,r)})),V=O((function(e,n){return b(_(e).appendChild(_(n)))})),z=function(e){return _(e)instanceof HTMLCanvasElement},Q=function(e,n){_(e).width=n>>>0},Y=function(e,n){_(e).height=n>>>0},Z=O((function(e,n,t){var r=_(e).getContext(d(n,t));return p(r)?0:b(r)})),ee=O((function(e,n){return b(Reflect.get(_(e),_(n)))})),ne=O((function(e,n){return b(_(e).call(_(n)))})),te=function(e,n){return b(new Function(d(e,n)))},re=O((function(){return b(self.self)})),oe=O((function(){return b(window.window)})),ae=O((function(){return b(globalThis.globalThis)})),ce=O((function(){return b(t.g.global)})),_e=function(e){return void 0===_(e)},ie=function(e,n){const t=_(n);var r="number"==typeof t?t:void 0;(null!==g&&g.buffer===o.memory.buffer||(g=new Float64Array(o.memory.buffer)),g)[e/8+1]=p(r)?0:r,m()[e/4+0]=!p(r)},ue=function(e,n){var t=x(h(_(n)),o.__wbindgen_malloc,o.__wbindgen_realloc),r=y;m()[e/4+1]=r,m()[e/4+0]=t},le=function(e,n){throw new Error(d(e,n))},se=function(e,n,t){return b(k(e,n,24,E))},fe=function(e,n,t){return b(k(e,n,24,S))}}))},314:(e,n,t)=>{var r=([r])=>t.v(n,e.id,"9ac3f7d2021e74415483",{"./jackvm_player_bg.js":{__wbindgen_object_drop_ref:r.ug,__wbindgen_cb_drop:r.G6,__wbindgen_string_new:r.h4,__wbg_alert_f4ecf2eef6372361:r.L7,__wbindgen_object_clone_ref:r.m_,__wbg_new_59cb74e423758ede:r.h9,__wbg_stack_558ba5917b466edd:r.Dz,__wbg_error_4bb6c2a97407129a:r.kF,__wbg_instanceof_Window_49f532f06a9786ee:r.wp,__wbg_document_c0366b39e4f4c89a:r.NI,__wbg_requestAnimationFrame_ef0e2294dc8b1088:r.Cf,__wbg_body_c8cb19d760637268:r.AT,__wbg_createElement_99351c8bf0efac6e:r.Vl,__wbg_addEventListener_6a37bc32387cb66d:r.S0,__wbg_newwithu8clampedarray_32a8c1cd1f7ae859:r.SE,__wbg_log_f2e13ca55da8bad3:r.wl,__wbg_instanceof_HtmlElement_ed44c8f443dbd619:r.jm,__wbg_instanceof_CanvasRenderingContext2d_1d38418d1d6c8b34:r.GJ,__wbg_putImageData_5f6666a81288bdcf:r.Gr,__wbg_appendChild_7c45aeccd496f2a5:r.O8,__wbg_instanceof_HtmlCanvasElement_7bd3ee7838f11fc3:r.jL,__wbg_setwidth_1d0e975feecff3ef:r.Rs,__wbg_setheight_7758ee3ff5c65474:r.XJ,__wbg_getContext_3db9399e6dc524ff:r.HQ,__wbg_get_85e0a3b459845fe2:r.Mw,__wbg_call_951bd0c6d815d6f1:r.nM,__wbg_newnoargs_7c6bd521992b4022:r.$2,__wbg_self_6baf3a3aa7b63415:r.I1,__wbg_window_63fc4027b66c265b:r.Ic,__wbg_globalThis_513fb247e8e4e6d2:r.dr,__wbg_global_b87245cd886d7113:r.Zt,__wbindgen_is_undefined:r.XP,__wbindgen_number_get:r.M1,__wbindgen_debug_string:r.fY,__wbindgen_throw:r.Or,__wbindgen_closure_wrapper99:r.G4,__wbindgen_closure_wrapper101:r.rh}});t.a(e,(e=>{var n=e([t(304)]);return n.then?n.then(r):r(n)}),1)},85:()=>{},473:(e,n,t)=>{t.a(e,(async e=>{var n=t(160),r=e([n]);n=(r.then?await r:r)[0];const o=document.getElementById("screen-container");let a=new n.io(o),c=o.querySelector("canvas");["sm:w-512px","sm:h-256px","lg:w-1024px","lg:h-512px"].forEach((e=>{c.classList.add(e)})),document.querySelector("#editor").addEventListener("change",(e=>{updateProgram()}));let _=document.querySelector("#play-overlay");_.addEventListener("click",(e=>(console.log("player clicked"),a.isStopped()&&(a.restart(),_.classList.toggle("hidden")),!1))),a.addHaltListener((()=>{_.classList.toggle("hidden")})),window.dispatchEvent(new CustomEvent("JackVmPlayerLoaded",{detail:a}))}))}},c={};function _(e){if(c[e])return c[e].exports;var n=c[e]={id:e,loaded:!1,exports:{}};return a[e](n,n.exports,_),n.loaded=!0,n.exports}e="function"==typeof Symbol?Symbol("webpack then"):"__webpack_then__",n="function"==typeof Symbol?Symbol("webpack exports"):"__webpack_exports__",t=e=>{e&&(e.forEach((e=>e.r--)),e.forEach((e=>e.r--?e.r++:e())))},r=e=>!--e.r&&e(),o=(e,n)=>e?e.push(n):r(n),_.a=(a,c,_)=>{var i,u,l,s=_&&[],f=a.exports,d=!0,b=!1,p=(n,t,r)=>{b||(b=!0,t.r+=n.length,n.map(((n,o)=>{n[e](t,r)})),b=!1)},g=new Promise(((e,n)=>{l=n,u=()=>{e(f),t(s),s=0}}));g[n]=f,g[e]=(e,n)=>{if(d)return r(e);i&&p(i,e,n),o(s,e),g.catch(n)},a.exports=g,c((a=>{if(!a)return u();var c,_;i=(a=>a.map((a=>{if(null!==a&&"object"==typeof a){if(a[e])return a;if(a.then){var c=[];a.then((e=>{_[n]=e,t(c),c=0}));var _={[e]:(e,n)=>{o(c,e),a.catch(n)}};return _}}return{[e]:e=>{r(e)},[n]:a}})))(a);var l=new Promise(((e,t)=>{(c=()=>e(_=i.map((e=>e[n])))).r=0,p(i,c,t)}));return c.r?l:_})).then(u,l),d=!1},_.d=(e,n)=>{for(var t in n)_.o(n,t)&&!_.o(e,t)&&Object.defineProperty(e,t,{enumerable:!0,get:n[t]})},_.g=function(){if("object"==typeof globalThis)return globalThis;try{return this||new Function("return this")()}catch(e){if("object"==typeof window)return window}}(),_.hmd=e=>((e=Object.create(e)).children||(e.children=[]),Object.defineProperty(e,"exports",{enumerable:!0,set:()=>{throw new Error("ES Modules may not assign module.exports or exports.*, Use ESM export syntax, instead: "+e.id)}}),e),_.o=(e,n)=>Object.prototype.hasOwnProperty.call(e,n),(()=>{var e;_.g.importScripts&&(e=_.g.location+"");var n=_.g.document;if(!e&&n&&(n.currentScript&&(e=n.currentScript.src),!e)){var t=n.getElementsByTagName("script");t.length&&(e=t[t.length-1].src)}if(!e)throw new Error("Automatic publicPath is not supported in this browser");e=e.replace(/#.*$/,"").replace(/\?.*$/,"").replace(/\/[^\/]+$/,"/"),_.p=e})(),_.v=(e,n,t,r)=>{var o=fetch(_.p+""+t+".module.wasm");return"function"==typeof WebAssembly.instantiateStreaming?WebAssembly.instantiateStreaming(o,r).then((n=>Object.assign(e,n.instance.exports))):o.then((e=>e.arrayBuffer())).then((e=>WebAssembly.instantiate(e,r))).then((n=>Object.assign(e,n.instance.exports)))},_(473),_(85)})();
//# sourceMappingURL=index.9f207d360a401abc236a.js.map