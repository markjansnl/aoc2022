(function(){const n=document.createElement("link").relList;if(n&&n.supports&&n.supports("modulepreload"))return;for(const o of document.querySelectorAll('link[rel="modulepreload"]'))s(o);new MutationObserver(o=>{for(const r of o)if(r.type==="childList")for(const l of r.addedNodes)l.tagName==="LINK"&&l.rel==="modulepreload"&&s(l)}).observe(document,{childList:!0,subtree:!0});function t(o){const r={};return o.integrity&&(r.integrity=o.integrity),o.referrerpolicy&&(r.referrerPolicy=o.referrerpolicy),o.crossorigin==="use-credentials"?r.credentials="include":o.crossorigin==="anonymous"?r.credentials="omit":r.credentials="same-origin",r}function s(o){if(o.ep)return;o.ep=!0;const r=t(o);fetch(o.href,r)}})();const T={},Xe=(e,n)=>e===n,C=Symbol("solid-proxy"),ye=Symbol("solid-track"),te={equals:Xe};let Ge=Ce;const L=1,se=2,Be={owned:null,cleanups:null,context:null,owner:null};var I=null;let j=null,v=null,A=null,O=null,pe=0;function Q(e,n){const t=v,s=I,o=e.length===0,r=o?Be:{owned:null,cleanups:null,context:null,owner:n||s},l=o?e:()=>e(()=>H(()=>me(r)));I=r,v=null;try{return K(l,!0)}finally{v=t,I=s}}function U(e,n){n=n?Object.assign({},te,n):te;const t={value:e,observers:null,observerSlots:null,comparator:n.equals||void 0},s=o=>(typeof o=="function"&&(o=o(t.value)),Ee(t,o));return[Te.bind(t),s]}function B(e,n,t){const s=je(e,n,!1,L);fe(s)}function N(e,n,t){t=t?Object.assign({},te,t):te;const s=je(e,n,!0,0);return s.observers=null,s.observerSlots=null,s.comparator=t.equals||void 0,fe(s),Te.bind(s)}function Ye(e){return K(e,!1)}function H(e){const n=v;v=null;try{return e()}finally{v=n}}function Qe(e){return I===null||(I.cleanups===null?I.cleanups=[e]:I.cleanups.push(e)),e}function Pe(){return v}function ze(e){const n=N(e),t=N(()=>_e(n()));return t.toArray=()=>{const s=t();return Array.isArray(s)?s:s!=null?[s]:[]},t}function Te(){const e=j;if(this.sources&&(this.state||e))if(this.state===L||e)fe(this);else{const n=A;A=null,K(()=>oe(this),!1),A=n}if(v){const n=this.observers?this.observers.length:0;v.sources?(v.sources.push(this),v.sourceSlots.push(n)):(v.sources=[this],v.sourceSlots=[n]),this.observers?(this.observers.push(v),this.observerSlots.push(v.sources.length-1)):(this.observers=[v],this.observerSlots=[v.sources.length-1])}return this.value}function Ee(e,n,t){let s=e.value;return(!e.comparator||!e.comparator(s,n))&&(e.value=n,e.observers&&e.observers.length&&K(()=>{for(let o=0;o<e.observers.length;o+=1){const r=e.observers[o],l=j&&j.running;l&&j.disposed.has(r),(l&&!r.tState||!l&&!r.state)&&(r.pure?A.push(r):O.push(r),r.observers&&Ne(r)),l||(r.state=L)}if(A.length>1e6)throw A=[],new Error},!1)),n}function fe(e){if(!e.fn)return;me(e);const n=I,t=v,s=pe;v=I=e,Je(e,e.value,s),v=t,I=n}function Je(e,n,t){let s;try{s=e.fn(n)}catch(o){e.pure&&(e.state=L),Le(o)}(!e.updatedAt||e.updatedAt<=t)&&(e.updatedAt!=null&&"observers"in e?Ee(e,s):e.value=s,e.updatedAt=t)}function je(e,n,t,s=L,o){const r={fn:e,state:s,updatedAt:null,owned:null,sources:null,sourceSlots:null,cleanups:null,value:n,owner:I,context:null,pure:t};return I===null||I!==Be&&(I.owned?I.owned.push(r):I.owned=[r]),r}function Oe(e){const n=j;if(e.state===0||n)return;if(e.state===se||n)return oe(e);if(e.suspense&&H(e.suspense.inFallback))return e.suspense.effects.push(e);const t=[e];for(;(e=e.owner)&&(!e.updatedAt||e.updatedAt<pe);)(e.state||n)&&t.push(e);for(let s=t.length-1;s>=0;s--)if(e=t[s],e.state===L||n)fe(e);else if(e.state===se||n){const o=A;A=null,K(()=>oe(e,t[0]),!1),A=o}}function K(e,n){if(A)return e();let t=!1;n||(A=[]),O?t=!0:O=[],pe++;try{const s=e();return Ze(t),s}catch(s){A||(O=null),Le(s)}}function Ze(e){if(A&&(Ce(A),A=null),e)return;const n=O;O=null,n.length&&K(()=>Ge(n),!1)}function Ce(e){for(let n=0;n<e.length;n++)Oe(e[n])}function oe(e,n){const t=j;e.state=0;for(let s=0;s<e.sources.length;s+=1){const o=e.sources[s];o.sources&&(o.state===L||t?o!==n&&Oe(o):(o.state===se||t)&&oe(o,n))}}function Ne(e){const n=j;for(let t=0;t<e.observers.length;t+=1){const s=e.observers[t];(!s.state||n)&&(s.state=se,s.pure?A.push(s):O.push(s),s.observers&&Ne(s))}}function me(e){let n;if(e.sources)for(;e.sources.length;){const t=e.sources.pop(),s=e.sourceSlots.pop(),o=t.observers;if(o&&o.length){const r=o.pop(),l=t.observerSlots.pop();s<o.length&&(r.sourceSlots[l]=s,o[s]=r,t.observerSlots[s]=l)}}if(e.owned){for(n=0;n<e.owned.length;n++)me(e.owned[n]);e.owned=null}if(e.cleanups){for(n=0;n<e.cleanups.length;n++)e.cleanups[n]();e.cleanups=null}e.state=0,e.context=null}function en(e){return e instanceof Error||typeof e=="string"?e:new Error("Unknown error")}function Le(e){throw e=en(e),e}function _e(e){if(typeof e=="function"&&!e.length)return _e(e());if(Array.isArray(e)){const n=[];for(let t=0;t<e.length;t++){const s=_e(e[t]);Array.isArray(s)?n.push.apply(n,s):n.push(s)}return n}return e}const nn=Symbol("fallback");function be(e){for(let n=0;n<e.length;n++)e[n]()}function tn(e,n,t={}){let s=[],o=[],r=[],l=0,i=n.length>1?[]:null;return Qe(()=>be(r)),()=>{let u=e()||[],f,c;return u[ye],H(()=>{let h=u.length,g,w,k,p,S,d,b,$,y;if(h===0)l!==0&&(be(r),r=[],s=[],o=[],l=0,i&&(i=[])),t.fallback&&(s=[nn],o[0]=Q(R=>(r[0]=R,t.fallback())),l=1);else if(l===0){for(o=new Array(h),c=0;c<h;c++)s[c]=u[c],o[c]=Q(_);l=h}else{for(k=new Array(h),p=new Array(h),i&&(S=new Array(h)),d=0,b=Math.min(l,h);d<b&&s[d]===u[d];d++);for(b=l-1,$=h-1;b>=d&&$>=d&&s[b]===u[$];b--,$--)k[$]=o[b],p[$]=r[b],i&&(S[$]=i[b]);for(g=new Map,w=new Array($+1),c=$;c>=d;c--)y=u[c],f=g.get(y),w[c]=f===void 0?-1:f,g.set(y,c);for(f=d;f<=b;f++)y=s[f],c=g.get(y),c!==void 0&&c!==-1?(k[c]=o[f],p[c]=r[f],i&&(S[c]=i[f]),c=w[c],g.set(y,c)):r[f]();for(c=d;c<h;c++)c in k?(o[c]=k[c],r[c]=p[c],i&&(i[c]=S[c],i[c](c))):o[c]=Q(_);o=o.slice(0,l=h),s=u.slice(0)}return o});function _(h){if(r[c]=h,i){const[g,w]=U(c);return i[c]=w,n(u[c],g)}return n(u[c])}}}function M(e,n){return H(()=>e(n||{}))}function $e(e){const n="fallback"in e&&{fallback:()=>e.fallback};return N(tn(()=>e.each,e.children,n||void 0))}function sn(e){let n=!1;const t=e.keyed,s=N(()=>e.when,void 0,{equals:(o,r)=>n?o===r:!o==!r});return N(()=>{const o=s();if(o){const r=e.children,l=typeof r=="function"&&r.length>0;return n=t||l,l?H(()=>r(o)):r}return e.fallback},void 0,void 0)}function on(e){let n=!1,t=!1;const s=(l,i)=>l[0]===i[0]&&(n?l[1]===i[1]:!l[1]==!i[1])&&l[2]===i[2],o=ze(()=>e.children),r=N(()=>{let l=o();Array.isArray(l)||(l=[l]);for(let i=0;i<l.length;i++){const u=l[i].when;if(u)return t=!!l[i].keyed,[i,u,l[i]]}return[-1]},void 0,{equals:s});return N(()=>{const[l,i,u]=r();if(l<0)return e.fallback;const f=u.children,c=typeof f=="function"&&f.length>0;return n=t||c,c?H(()=>f(i)):f},void 0,void 0)}function Ie(e){return e}function rn(e,n,t){let s=t.length,o=n.length,r=s,l=0,i=0,u=n[o-1].nextSibling,f=null;for(;l<o||i<r;){if(n[l]===t[i]){l++,i++;continue}for(;n[o-1]===t[r-1];)o--,r--;if(o===l){const c=r<s?i?t[i-1].nextSibling:t[r-i]:u;for(;i<r;)e.insertBefore(t[i++],c)}else if(r===i)for(;l<o;)(!f||!f.has(n[l]))&&n[l].remove(),l++;else if(n[l]===t[r-1]&&t[i]===n[o-1]){const c=n[--o].nextSibling;e.insertBefore(t[i++],n[l++].nextSibling),e.insertBefore(t[--r],c),n[o]=t[r]}else{if(!f){f=new Map;let _=i;for(;_<r;)f.set(t[_],_++)}const c=f.get(n[l]);if(c!=null)if(i<c&&c<r){let _=l,h=1,g;for(;++_<o&&_<r&&!((g=f.get(n[_]))==null||g!==c+h);)h++;if(h>c-i){const w=n[l];for(;i<c;)e.insertBefore(t[i++],w)}else e.replaceChild(t[i++],n[l++])}else l++;else n[l++].remove()}}}const Ae="_$DX_DELEGATE";function ln(e,n,t,s={}){let o;return Q(r=>{o=r,n===document?e():X(n,e(),n.firstChild?null:void 0,t)},s.owner),()=>{o(),n.textContent=""}}function V(e,n,t){const s=document.createElement("template");s.innerHTML=e;let o=s.content.firstChild;return t&&(o=o.firstChild),o}function un(e,n=window.document){const t=n[Ae]||(n[Ae]=new Set);for(let s=0,o=e.length;s<o;s++){const r=e[s];t.has(r)||(t.add(r),n.addEventListener(r,fn))}}function ke(e,n,t){t==null?e.removeAttribute(n):e.setAttribute(n,t)}function x(e,n){n==null?e.removeAttribute("class"):e.className=n}function cn(e,n,t={}){const s=Object.keys(n||{}),o=Object.keys(t);let r,l;for(r=0,l=o.length;r<l;r++){const i=o[r];!i||i==="undefined"||n[i]||(Se(e,i,!1),delete t[i])}for(r=0,l=s.length;r<l;r++){const i=s[r],u=!!n[i];!i||i==="undefined"||t[i]===u||!u||(Se(e,i,!0),t[i]=u)}return t}function de(e,n,t){if(!n)return t?ke(e,"style"):n;const s=e.style;if(typeof n=="string")return s.cssText=n;typeof t=="string"&&(s.cssText=t=void 0),t||(t={}),n||(n={});let o,r;for(r in t)n[r]==null&&s.removeProperty(r),delete t[r];for(r in n)o=n[r],o!==t[r]&&(s.setProperty(r,o),t[r]=o);return t}function X(e,n,t,s){if(t!==void 0&&!s&&(s=[]),typeof n!="function")return re(e,n,s,t);B(o=>re(e,n(),o,t),s)}function Se(e,n,t){const s=n.trim().split(/\s+/);for(let o=0,r=s.length;o<r;o++)e.classList.toggle(s[o],t)}function fn(e){const n=`$$${e.type}`;let t=e.composedPath&&e.composedPath()[0]||e.target;for(e.target!==t&&Object.defineProperty(e,"target",{configurable:!0,value:t}),Object.defineProperty(e,"currentTarget",{configurable:!0,get(){return t||document}}),T.registry&&!T.done&&(T.done=!0,document.querySelectorAll("[id^=pl-]").forEach(s=>s.remove()));t;){const s=t[n];if(s&&!t.disabled){const o=t[`${n}Data`];if(o!==void 0?s.call(t,o,e):s.call(t,e),e.cancelBubble)return}t=t._$host||t.parentNode||t.host}}function re(e,n,t,s,o){for(T.context&&!t&&(t=[...e.childNodes]);typeof t=="function";)t=t();if(n===t)return t;const r=typeof n,l=s!==void 0;if(e=l&&t[0]&&t[0].parentNode||e,r==="string"||r==="number"){if(T.context)return t;if(r==="number"&&(n=n.toString()),l){let i=t[0];i&&i.nodeType===3?i.data=n:i=document.createTextNode(n),t=F(e,t,s,i)}else t!==""&&typeof t=="string"?t=e.firstChild.data=n:t=e.textContent=n}else if(n==null||r==="boolean"){if(T.context)return t;t=F(e,t,s)}else{if(r==="function")return B(()=>{let i=n();for(;typeof i=="function";)i=i();t=re(e,i,t,s)}),()=>t;if(Array.isArray(n)){const i=[],u=t&&Array.isArray(t);if(he(i,n,t,o))return B(()=>t=re(e,i,t,s,!0)),()=>t;if(T.context){if(!i.length)return t;for(let f=0;f<i.length;f++)if(i[f].parentNode)return t=i}if(i.length===0){if(t=F(e,t,s),l)return t}else u?t.length===0?Me(e,i,s):rn(e,t,i):(t&&F(e),Me(e,i));t=i}else if(n instanceof Node){if(T.context&&n.parentNode)return t=l?[n]:n;if(Array.isArray(t)){if(l)return t=F(e,t,s,n);F(e,t,null,n)}else t==null||t===""||!e.firstChild?e.appendChild(n):e.replaceChild(n,e.firstChild);t=n}}return t}function he(e,n,t,s){let o=!1;for(let r=0,l=n.length;r<l;r++){let i=n[r],u=t&&t[r];if(i instanceof Node)e.push(i);else if(!(i==null||i===!0||i===!1))if(Array.isArray(i))o=he(e,i,u)||o;else if(typeof i=="function")if(s){for(;typeof i=="function";)i=i();o=he(e,Array.isArray(i)?i:[i],Array.isArray(u)?u:[u])||o}else e.push(i),o=!0;else{const f=String(i);u&&u.nodeType===3&&u.data===f?e.push(u):e.push(document.createTextNode(f))}}return o}function Me(e,n,t=null){for(let s=0,o=n.length;s<o;s++)e.insertBefore(n[s],t)}function F(e,n,t,s){if(t===void 0)return e.textContent="";const o=s||document.createTextNode("");if(n.length){let r=!1;for(let l=n.length-1;l>=0;l--){const i=n[l];if(o!==i){const u=i.parentNode===e;!r&&!l?u?e.replaceChild(o,i):e.insertBefore(o,t):u&&i.remove()}else r=!0}}else e.insertBefore(o,t);return[o]}const ie=Symbol("store-raw"),Y=Symbol("store-node"),an=Symbol("store-name");function Re(e,n){let t=e[C];if(!t&&(Object.defineProperty(e,C,{value:t=new Proxy(e,_n)}),!Array.isArray(e))){const s=Object.keys(e),o=Object.getOwnPropertyDescriptors(e);for(let r=0,l=s.length;r<l;r++){const i=s[r];if(o[i].get){const u=o[i].get.bind(t);Object.defineProperty(e,i,{enumerable:o[i].enumerable,get:u})}}}return t}function D(e){let n;return e!=null&&typeof e=="object"&&(e[C]||!(n=Object.getPrototypeOf(e))||n===Object.prototype||Array.isArray(e))}function q(e,n=new Set){let t,s,o,r;if(t=e!=null&&e[ie])return t;if(!D(e)||n.has(e))return e;if(Array.isArray(e)){Object.isFrozen(e)?e=e.slice(0):n.add(e);for(let l=0,i=e.length;l<i;l++)o=e[l],(s=q(o,n))!==o&&(e[l]=s)}else{Object.isFrozen(e)?e=Object.assign({},e):n.add(e);const l=Object.keys(e),i=Object.getOwnPropertyDescriptors(e);for(let u=0,f=l.length;u<f;u++)r=l[u],!i[r].get&&(o=e[r],(s=q(o,n))!==o&&(e[r]=s))}return e}function ve(e){let n=e[Y];return n||Object.defineProperty(e,Y,{value:n={}}),n}function ge(e,n,t){return e[n]||(e[n]=Ue(t))}function dn(e,n){const t=Reflect.getOwnPropertyDescriptor(e,n);return!t||t.get||!t.configurable||n===C||n===Y||n===an||(delete t.value,delete t.writable,t.get=()=>e[C][n]),t}function Fe(e){if(Pe()){const n=ve(e);(n._||(n._=Ue()))()}}function yn(e){return Fe(e),Reflect.ownKeys(e)}function Ue(e){const[n,t]=U(e,{equals:!1,internal:!0});return n.$=t,n}const _n={get(e,n,t){if(n===ie)return e;if(n===C)return t;if(n===ye)return Fe(e),t;const s=ve(e),o=s.hasOwnProperty(n);let r=o?s[n]():e[n];if(n===Y||n==="__proto__")return r;if(!o){const l=Object.getOwnPropertyDescriptor(e,n);Pe()&&(typeof r!="function"||e.hasOwnProperty(n))&&!(l&&l.get)&&(r=ge(s,n,r)())}return D(r)?Re(r):r},has(e,n){return n===ie||n===C||n===ye||n===Y||n==="__proto__"?!0:(this.get(e,n,e),n in e)},set(){return!0},deleteProperty(){return!0},ownKeys:yn,getOwnPropertyDescriptor:dn};function W(e,n,t,s=!1){if(!s&&e[n]===t)return;const o=e[n],r=e.length;t===void 0?delete e[n]:e[n]=t;let l=ve(e),i;(i=ge(l,n,o))&&i.$(()=>t),Array.isArray(e)&&e.length!==r&&(i=ge(l,"length",r))&&i.$(e.length),(i=l._)&&i.$()}function De(e,n){const t=Object.keys(n);for(let s=0;s<t.length;s+=1){const o=t[s];W(e,o,n[o])}}function hn(e,n){if(typeof n=="function"&&(n=n(e)),n=q(n),Array.isArray(n)){if(e===n)return;let t=0,s=n.length;for(;t<s;t++){const o=n[t];e[t]!==o&&W(e,t,o)}W(e,"length",s)}else De(e,n)}function G(e,n,t=[]){let s,o=e;if(n.length>1){s=n.shift();const l=typeof s,i=Array.isArray(e);if(Array.isArray(s)){for(let u=0;u<s.length;u++)G(e,[s[u]].concat(n),t);return}else if(i&&l==="function"){for(let u=0;u<e.length;u++)s(e[u],u)&&G(e,[u].concat(n),t);return}else if(i&&l==="object"){const{from:u=0,to:f=e.length-1,by:c=1}=s;for(let _=u;_<=f;_+=c)G(e,[_].concat(n),t);return}else if(n.length>1){G(e[s],n,[s].concat(t));return}o=e[s],t=[s].concat(t)}let r=n[0];typeof r=="function"&&(r=r(o,t),r===o)||s===void 0&&r==null||(r=q(r),s===void 0||D(o)&&D(r)&&!Array.isArray(r)?De(o,r):W(e,s,r))}function gn(...[e,n]){const t=q(e||{}),s=Array.isArray(t),o=Re(t);function r(...l){Ye(()=>{s&&l.length===1?hn(t,l[0]):G(t,l)})}return[o,r]}const le=new WeakMap,qe={get(e,n){if(n===ie)return e;const t=e[n];let s;return D(t)?le.get(t)||(le.set(t,s=new Proxy(t,qe)),s):t},set(e,n,t){return W(e,n,q(t)),!0},deleteProperty(e,n){return W(e,n,void 0,!0),!0}};function P(e){return n=>{if(D(n)){let t;(t=le.get(n))||le.set(n,t=new Proxy(n,qe)),e(t)}return n}}let a;const We=new TextDecoder("utf-8",{ignoreBOM:!0,fatal:!0});We.decode();let z=new Uint8Array;function J(){return z.byteLength===0&&(z=new Uint8Array(a.memory.buffer)),z}function He(e,n){return We.decode(J().subarray(e,e+n))}let Z=new Int32Array;function ue(){return Z.byteLength===0&&(Z=new Int32Array(a.memory.buffer)),Z}function wn(){try{const t=a.__wbindgen_add_to_stack_pointer(-16);a.example(t);var e=ue()[t/4+0],n=ue()[t/4+1];return He(e,n)}finally{a.__wbindgen_add_to_stack_pointer(16),a.__wbindgen_free(e,n)}}let we=0;const ee=new TextEncoder("utf-8"),pn=typeof ee.encodeInto=="function"?function(e,n){return ee.encodeInto(e,n)}:function(e,n){const t=ee.encode(e);return n.set(t),{read:e.length,written:t.length}};function mn(e,n,t){if(t===void 0){const i=ee.encode(e),u=n(i.length);return J().subarray(u,u+i.length).set(i),we=i.length,u}let s=e.length,o=n(s);const r=J();let l=0;for(;l<s;l++){const i=e.charCodeAt(l);if(i>127)break;r[o+l]=i}if(l!==s){l!==0&&(e=e.slice(l)),o=t(o,s,s=l+e.length*3);const i=J().subarray(o+l,o+s),u=pn(e,i);l+=u.written}return we=l,o}let ne=new Uint32Array;function kn(){return ne.byteLength===0&&(ne=new Uint32Array(a.memory.buffer)),ne}function vn(e,n){return kn().subarray(e/4,e/4+n)}class ce{static __wrap(n){const t=Object.create(ce.prototype);return t.ptr=n,t}__destroy_into_raw(){const n=this.ptr;return this.ptr=0,n}free(){const n=this.__destroy_into_raw();a.__wbg_keepaway_free(n)}get round(){return a.__wbg_get_keepaway_round(this.ptr)>>>0}set round(n){a.__wbg_set_keepaway_round(this.ptr,n)}get monkey_index(){return a.__wbg_get_keepaway_monkey_index(this.ptr)>>>0}set monkey_index(n){a.__wbg_set_keepaway_monkey_index(this.ptr,n)}constructor(n){const t=mn(n,a.__wbindgen_malloc,a.__wbindgen_realloc),s=we,o=a.keepaway_new(t,s);return ce.__wrap(o)}monkey_count(){return a.keepaway_monkey_count(this.ptr)>>>0}monkey_items(n){try{const r=a.__wbindgen_add_to_stack_pointer(-16);a.keepaway_monkey_items(r,this.ptr,n);var t=ue()[r/4+0],s=ue()[r/4+1],o=vn(t,s).slice();return a.__wbindgen_free(t,s*4),o}finally{a.__wbindgen_add_to_stack_pointer(16)}}inspect(){return a.keepaway_inspect(this.ptr)>>>0}get_borred(){return a.keepaway_get_borred(this.ptr)>>>0}throw(){return a.keepaway_throw(this.ptr)>>>0}next(){a.keepaway_next(this.ptr)}monkey_business(){return a.keepaway_monkey_business(this.ptr)>>>0}}async function bn(e,n){if(typeof Response=="function"&&e instanceof Response){if(typeof WebAssembly.instantiateStreaming=="function")try{return await WebAssembly.instantiateStreaming(e,n)}catch(s){if(e.headers.get("Content-Type")!="application/wasm")console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n",s);else throw s}const t=await e.arrayBuffer();return await WebAssembly.instantiate(t,n)}else{const t=await WebAssembly.instantiate(e,n);return t instanceof WebAssembly.Instance?{instance:t,module:e}:t}}function $n(){const e={};return e.wbg={},e.wbg.__wbindgen_throw=function(n,t){throw new Error(He(n,t))},e}function In(e,n){return a=e.exports,Ke.__wbindgen_wasm_module=n,Z=new Int32Array,ne=new Uint32Array,z=new Uint8Array,a}async function Ke(e){typeof e>"u"&&(e=new URL("/assets/wasm_bg.d55f9b55.wasm",self.location));const n=$n();(typeof e=="string"||typeof Request=="function"&&e instanceof Request||typeof URL=="function"&&e instanceof URL)&&(e=fetch(e));const{instance:t,module:s}=await bn(await e,n);return In(t,s)}const An="/assets/catcher.6b15c0cf.png",Sn="/assets/monkey.3aa9c8c1.png",Mn="_App_1wtjp_8",xn="_RoundInfo_1wtjp_17",Bn="_Catcher_1wtjp_29",Pn="_Monkey_1wtjp_38",Tn="_MonkeyInfo_1wtjp_46",En="_MonkeyInfoBusy_1wtjp_55",jn="_MonkeyInfoFirst_1wtjp_63",On="_MonkeyInfoSecond_1wtjp_71",Cn="_MonkeyInfoBusinessHidden_1wtjp_79",Nn="_MonkeyInfoBusiness_1wtjp_79",Ln="_MonkeyInfoHidden_1wtjp_99",Rn="_Item_1wtjp_107",Fn="_Input_1wtjp_120",Un="_InputFooter_1wtjp_137",Dn="_InputRangeSpan_1wtjp_144",qn="_InputRangeSlider_1wtjp_151",Wn="_InputStartButton_1wtjp_155",Hn="_InputTextArea_1wtjp_166",Kn="_MonkeyBusinessStrip_1wtjp_175",Vn="_MonkeyBusinessTimes_1wtjp_188",Xn="_MonkeyBusinessEquals_1wtjp_198",m={App:Mn,RoundInfo:xn,Catcher:Bn,Monkey:Pn,MonkeyInfo:Tn,MonkeyInfoBusy:En,MonkeyInfoFirst:jn,MonkeyInfoSecond:On,MonkeyInfoBusinessHidden:Cn,MonkeyInfoBusiness:Nn,MonkeyInfoHidden:Ln,Item:Rn,Input:Fn,InputFooter:Un,InputRangeSpan:Dn,InputRangeSlider:qn,InputStartButton:Wn,InputTextArea:Hn,MonkeyBusinessStrip:Kn,MonkeyBusinessTimes:Vn,MonkeyBusinessEquals:Xn},ae=V("<div></div>"),Gn=V("<div>Round </div>"),Ve=V("<img>"),Yn=V('<div><textarea></textarea><div><span>Speed:<input type="range" min="0" max="3"></span><button>Start</button></div></div>'),Qn=V("<div>&times;</div>"),zn=V("<div> = </div>"),E=e=>e===0?0:new Promise(n=>setTimeout(n,e)),Jn=()=>{const[e,n]=U(!1),[t,s]=U(!0),[o,r]=U(""),[l,i]=U(1),[u,f]=gn({round:1,catcher_flipped:!1,monkies:[],items:[],monkeyBusiness:0});let c;Ke().then(()=>{n(!0),r(wn())});const _=()=>Math.pow(3-l(),3)*10,h=()=>{s(!1),c=new ce(o());const g=[],w=Array.from(new Array(c.monkey_count())).map((k,p,S)=>{const d=c?Array.from(c.monkey_items(p)).map((b,$)=>(g.push({pos:p/S.length,index:$+1,worryLevel:b}),g.length-1)):[];return{pos:p/S.length,items:d,itemsInspected:0,ranking:0}});f({catcher_flipped:!1,monkies:w,items:g}),(async()=>{for(let k=0;k<20;k++)for(let p=0;p<u.monkies.length;p++){f(P(d=>{d.round=k+1,d.catcher_flipped=d.monkies[p].pos>.25&&d.monkies[p].pos<=.75}));const S=u.monkies[p].items.length;for(let d=0;d<S;d++){f(P(y=>{for(let R=0;R<y.monkies[p].items.length;R++)y.items[y.monkies[p].items[R]].index=R})),await E(_()),f(P(y=>{y.items[u.monkies[p].items[0]].worryLevel=c.inspect(),y.monkies[p].itemsInspected++})),await E(.5*_()),f(P(y=>y.items[y.monkies[p].items[0]].worryLevel=c.get_borred())),await E(.5*_());let b=0,$=0;f(P(y=>{b=c.throw(),$=y.monkies[p].items.splice(0,1)[0],y.monkies[b].items.push($),y.items[$].pos=y.monkies[b].pos,y.items[$].index=0})),await E(_()),f(P(y=>{y.items[$].index=y.monkies[b].items.length})),await E(_())}c.next()}await E(1e3),f(P(k=>{k.monkies.map((S,d)=>[d,S.itemsInspected]).sort((S,d)=>d[1]-S[1]).forEach(([S,d],b)=>k.monkies[S].ranking=b+1)})),await E(1e3),f(P(k=>{k.monkeyBusiness=c.monkey_business()}))})()};return(()=>{const g=ae.cloneNode(!0);return X(g,M(sn,{get when(){return e()},get children(){return M(on,{get children(){return[M(Ie,{get when(){return t()},get children(){return M(st,{input:o,setInput:r,speed:l,setSpeed:i,onStart:h})}}),M(Ie,{get when(){return!t()},get children(){return[M(Zn,{get round(){return u.round}}),M(et,{get flipped(){return u.catcher_flipped}}),M($e,{get each(){return u.monkies},children:(w,k)=>[M(nt,{get pos(){return w.pos}}),M(xe,{get pos(){return w.pos},get itemsInspected(){return w.itemsInspected},get ranking(){return w.ranking}})]}),M(xe,{pos:0,get itemsInspected(){return u.monkeyBusiness},ranking:-1}),M(ot,{get monkeyBusiness(){return u.monkeyBusiness}}),M($e,{get each(){return u.items},children:(w,k)=>M(tt,{get pos(){return w.pos},get index(){return w.index},get worryLevel(){return w.worryLevel},get delay(){return _()}})})]}})]}})}})),B(()=>x(g,m.App)),g})()},Zn=e=>(()=>{const n=Gn.cloneNode(!0);return n.firstChild,X(n,()=>e.round,null),B(()=>x(n,m.RoundInfo)),n})(),et=e=>(()=>{const n=Ve.cloneNode(!0);return ke(n,"src",An),B(t=>{const s=m.Catcher,o=`scaleX(${e.flipped?-1:1})`;return s!==t._v$&&x(n,t._v$=s),o!==t._v$2&&n.style.setProperty("transform",t._v$2=o),t},{_v$:void 0,_v$2:void 0}),n})(),nt=e=>(()=>{const n=Ve.cloneNode(!0);return ke(n,"src",Sn),B(t=>{const s=m.Monkey,o=`scaleX(${e.pos<.25||e.pos>.75?-1:1})`,r=`${25*Math.sin(e.pos*2*Math.PI)}vh`,l=`${25*Math.cos(e.pos*2*Math.PI)}vh`;return s!==t._v$3&&x(n,t._v$3=s),o!==t._v$4&&n.style.setProperty("transform",t._v$4=o),r!==t._v$5&&n.style.setProperty("top",t._v$5=r),l!==t._v$6&&n.style.setProperty("left",t._v$6=l),t},{_v$3:void 0,_v$4:void 0,_v$5:void 0,_v$6:void 0}),n})(),xe=e=>(()=>{const n=ae.cloneNode(!0);return X(n,()=>e.itemsInspected),B(t=>{const s=m.MonkeyInfo,o={[m.MonkeyInfoBusy]:e.ranking===0,[m.MonkeyInfoFirst]:e.ranking===1,[m.MonkeyInfoSecond]:e.ranking===2,[m.MonkeyInfoHidden]:e.ranking>2,[m.MonkeyInfoBusinessHidden]:e.ranking===-1&&e.itemsInspected===0,[m.MonkeyInfoBusiness]:e.ranking===-1&&e.itemsInspected>0},r=`${25*Math.sin(e.pos*2*Math.PI)}vh`,l=`${25*Math.cos(e.pos*2*Math.PI)}vh`;return s!==t._v$7&&x(n,t._v$7=s),t._v$8=cn(n,o,t._v$8),r!==t._v$9&&n.style.setProperty("--top",t._v$9=r),l!==t._v$10&&n.style.setProperty("--left",t._v$10=l),t},{_v$7:void 0,_v$8:void 0,_v$9:void 0,_v$10:void 0}),n})(),tt=e=>(()=>{const n=ae.cloneNode(!0);return X(n,()=>e.worryLevel),B(t=>{const s=m.Item,o=`${e.index===0?e.pos<.25||e.pos>.75?"-8vh":"-1vh":"-50% -50%"} 1vh`,r=`${((e.index===0?0:5)+25+6*e.index)*Math.cos(e.pos*2*Math.PI)}vh`,l=`${((e.index===0?0:6)+25+3*e.index)*Math.sin(e.pos*2*Math.PI)}vh`,i=100-e.index,u=`all ${e.delay/1e3}s ease-out`;return s!==t._v$11&&x(n,t._v$11=s),o!==t._v$12&&n.style.setProperty("translate",t._v$12=o),r!==t._v$13&&n.style.setProperty("left",t._v$13=r),l!==t._v$14&&n.style.setProperty("top",t._v$14=l),i!==t._v$15&&n.style.setProperty("z-index",t._v$15=i),u!==t._v$16&&n.style.setProperty("transition",t._v$16=u),t},{_v$11:void 0,_v$12:void 0,_v$13:void 0,_v$14:void 0,_v$15:void 0,_v$16:void 0}),n})(),st=e=>{const n=()=>["Slow","Normal","Fast","The Flash"][e.speed()];return(()=>{const t=Yn.cloneNode(!0),s=t.firstChild,o=s.nextSibling,r=o.firstChild,l=r.firstChild,i=l.nextSibling,u=r.nextSibling;return s.$$input=f=>e.setInput(f.currentTarget.value),i.$$input=f=>e.setSpeed(f.currentTarget.value),X(r,n,null),u.$$click=f=>e.onStart(),B(f=>{const c=m.Input,_=m.InputTextArea,h=m.InputFooter,g=m.InputRangeSpan,w=m.InputRangeSlider,k=m.InputStartButton;return c!==f._v$17&&x(t,f._v$17=c),_!==f._v$18&&x(s,f._v$18=_),h!==f._v$19&&x(o,f._v$19=h),g!==f._v$20&&x(r,f._v$20=g),w!==f._v$21&&x(i,f._v$21=w),k!==f._v$22&&x(u,f._v$22=k),f},{_v$17:void 0,_v$18:void 0,_v$19:void 0,_v$20:void 0,_v$21:void 0,_v$22:void 0}),B(()=>s.value=e.input()),B(()=>i.value=e.speed()),t})()},ot=e=>{const n=()=>({opacity:e.monkeyBusiness===0?0:.8}),t=()=>({opacity:e.monkeyBusiness===0?0:1});return[(()=>{const s=ae.cloneNode(!0);return B(o=>{const r=m.MonkeyBusinessStrip,l=n();return r!==o._v$23&&x(s,o._v$23=r),o._v$24=de(s,l,o._v$24),o},{_v$23:void 0,_v$24:void 0}),s})(),(()=>{const s=Qn.cloneNode(!0);return B(o=>{const r=m.MonkeyBusinessTimes,l=t();return r!==o._v$25&&x(s,o._v$25=r),o._v$26=de(s,l,o._v$26),o},{_v$25:void 0,_v$26:void 0}),s})(),(()=>{const s=zn.cloneNode(!0);return B(o=>{const r=m.MonkeyBusinessEquals,l=t();return r!==o._v$27&&x(s,o._v$27=r),o._v$28=de(s,l,o._v$28),o},{_v$27:void 0,_v$28:void 0}),s})()]};un(["input","click"]);ln(()=>M(Jn,{}),document.getElementById("root"));