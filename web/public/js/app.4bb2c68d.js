(function(e){function t(t){for(var r,a,c=t[0],s=t[1],l=t[2],d=0,p=[];d<c.length;d++)a=c[d],Object.prototype.hasOwnProperty.call(o,a)&&o[a]&&p.push(o[a][0]),o[a]=0;for(r in s)Object.prototype.hasOwnProperty.call(s,r)&&(e[r]=s[r]);u&&u(t);while(p.length)p.shift()();return i.push.apply(i,l||[]),n()}function n(){for(var e,t=0;t<i.length;t++){for(var n=i[t],r=!0,c=1;c<n.length;c++){var s=n[c];0!==o[s]&&(r=!1)}r&&(i.splice(t--,1),e=a(a.s=n[0]))}return e}var r={},o={app:0},i=[];function a(t){if(r[t])return r[t].exports;var n=r[t]={i:t,l:!1,exports:{}};return e[t].call(n.exports,n,n.exports,a),n.l=!0,n.exports}a.m=e,a.c=r,a.d=function(e,t,n){a.o(e,t)||Object.defineProperty(e,t,{enumerable:!0,get:n})},a.r=function(e){"undefined"!==typeof Symbol&&Symbol.toStringTag&&Object.defineProperty(e,Symbol.toStringTag,{value:"Module"}),Object.defineProperty(e,"__esModule",{value:!0})},a.t=function(e,t){if(1&t&&(e=a(e)),8&t)return e;if(4&t&&"object"===typeof e&&e&&e.__esModule)return e;var n=Object.create(null);if(a.r(n),Object.defineProperty(n,"default",{enumerable:!0,value:e}),2&t&&"string"!=typeof e)for(var r in e)a.d(n,r,function(t){return e[t]}.bind(null,r));return n},a.n=function(e){var t=e&&e.__esModule?function(){return e["default"]}:function(){return e};return a.d(t,"a",t),t},a.o=function(e,t){return Object.prototype.hasOwnProperty.call(e,t)},a.p="/";var c=window["webpackJsonp"]=window["webpackJsonp"]||[],s=c.push.bind(c);c.push=t,c=c.slice();for(var l=0;l<c.length;l++)t(c[l]);var u=s;i.push([0,"chunk-vendors"]),n()})({0:function(e,t,n){e.exports=n("56d7")},"0104":function(e,t,n){},1:function(e,t){},2:function(e,t){},"56d7":function(e,t,n){"use strict";n.r(t);n("e260"),n("e6cf"),n("cca6"),n("a79d");var r=n("2b0e"),o=function(){var e=this,t=e.$createElement,n=e._self._c||t;return n("div",{attrs:{id:"root"}},[n("SignedOut",{directives:[{name:"show",rawName:"v-show",value:!e.isSignedIn,expression:"!isSignedIn"}]}),n("SignedIn",{directives:[{name:"show",rawName:"v-show",value:e.isSignedIn,expression:"isSignedIn"}]})],1)},i=[],a=(n("99af"),n("0104"),n("db49")),c=n.n(a),s=function(){var e=this,t=e.$createElement,n=e._self._c||t;return n("main",[n("h1",[e._v("Welcome to FISHA!")]),n("p",[e._v(" FISHA in development and works in NEAR testnet chain. To start app you need to SignIn. The button below will sign you in using NEAR testnet Wallet (wallet.testnet.near.org). This works just like the main network (\"mainnet\") wallet, but the NEAR Tokens on testnet aren't convertible to other currencies - they're just for testing! ")]),n("p",[e._v("Go ahead and click the button below to try it out:")]),n("p",{staticStyle:{"text-align":"center","margin-top":"2.5em"}},[n("button",{on:{click:e.login}},[e._v("Sign in")])])])},l=[],u=n("1da1"),d=(n("96cf"),n("ac1f"),n("5319"),n("d9eb")),p=c()("testnet");function g(){return w.apply(this,arguments)}function w(){return w=Object(u["a"])(regeneratorRuntime.mark((function e(){var t;return regeneratorRuntime.wrap((function(e){while(1)switch(e.prev=e.next){case 0:return e.next=2,Object(d["connect"])(Object.assign({deps:{keyStore:new d["keyStores"].BrowserLocalStorageKeyStore}},p));case 2:return t=e.sent,window.walletConnection=new d["WalletConnection"](t),window.accountId=window.walletConnection.getAccountId(),e.next=7,new d["Contract"](window.walletConnection.account(),p.contractName,{viewMethods:["get_greeting"],changeMethods:["set_greeting"]});case 7:window.contract=e.sent;case 8:case"end":return e.stop()}}),e)}))),w.apply(this,arguments)}function f(){window.walletConnection.signOut(),window.location.replace(window.location.origin+window.location.pathname)}function h(){window.walletConnection.requestSignIn(p.contractName)}console.log(p);var v={name:"SignedOut",methods:{login:function(){console.log("calling utils.login"),h()}}},m=v,b=n("2877"),_=Object(b["a"])(m,s,l,!1,null,null,null),I=_.exports,y=function(){var e=this,t=e.$createElement,n=e._self._c||t;return n("div",[n("button",{staticClass:"link",staticStyle:{float:"right"},on:{click:e.logout}},[e._v("Sign out")]),n("main",[n("h1",[n("label",{staticStyle:{color:"var(--secondary)","border-bottom":"2px solid var(--secondary)"},attrs:{for:"greeting"}},[e._v(e._s(e.savedGreeting))]),e._v(" "+e._s(e.accountId)+" ")]),n("form",{on:{submit:function(t){return t.preventDefault(),e.saveGreeting.apply(null,arguments)}}},[n("fieldset",{ref:"fieldset"},[n("label",{staticStyle:{display:"block",color:"var(--gray)","margin-bottom":"0.5em"},attrs:{for:"greeting"}},[e._v("Change greeting")]),n("div",{staticStyle:{display:"flex"}},[n("input",{directives:[{name:"model",rawName:"v-model",value:e.newGreeting,expression:"newGreeting"}],staticStyle:{flex:"1"},attrs:{autocomplete:"off",id:"greeting"},domProps:{value:e.newGreeting},on:{input:function(t){t.target.composing||(e.newGreeting=t.target.value)}}}),n("button",{staticStyle:{"border-radius":"0 5px 5px 0"},attrs:{id:"save"}},[e._v("Save")])])])]),n("p",[e._v("Look at that! A Hello World app! This greeting is stored on the NEAR blockchain. Check it out:")]),n("hr"),n("p",[e._v(" If you have questions or issues - feel free to contact as by email: support@fisha.co.in ")])]),n("Notification",{directives:[{name:"show",rawName:"v-show",value:e.notificationVisible,expression:"notificationVisible"}],ref:"notification",attrs:{networkId:e.networkId,msg:"called method: set_greeting",contractId:e.contractId,visible:!1}})],1)},S=[],k=function(){var e=this,t=e.$createElement,n=e._self._c||t;return n("div",{staticClass:"notification"},[n("aside",[n("a",{attrs:{target:"_blank",rel:"noreferrer",href:e.urlPrefix+"/"+e.accountId}},[e._v(e._s(e.accountId))]),n("br"),e._v(" "+e._s(e.msg)+" "),n("br"),e._v("in contract: "),n("a",{attrs:{target:"_blank",rel:"noreferrer",href:e.urlPrefix+"/"+e.contractId}},[e._v(e._s(e.contractId))]),e._m(0)])])},x=[function(){var e=this,t=e.$createElement,n=e._self._c||t;return n("footer",[n("div",[e._v("✔ Succeeded")]),n("div",[e._v("Just now")])])}],O={name:"Notification",props:{networkId:String,msg:String,contractId:String},computed:{urlPrefix:function(){return"https://explorer."+this.networkId+".near.org/accounts"},accountId:function(){return window.accountId}}},N=O,j=Object(b["a"])(N,k,x,!1,null,null,null),C=j.exports,E={name:"SignedIn",beforeMount:function(){this.isSignedIn&&this.retrieveSavedGreeting()},components:{Notification:C},data:function(){return{savedGreeting:"",newGreeting:"",notificationVisible:!1}},computed:{isSignedIn:function(){return!!window.walletConnection&&window.walletConnection.isSignedIn()},accountId:function(){return window.accountId},contractId:function(){return window.contract?window.contract.contractId:null},networkId:function(){return window.networkId}},methods:{retrieveSavedGreeting:function(){var e=this;window.contract.get_greeting({account_id:window.accountId}).then((function(t){e.savedGreeting=t,e.newGreeting=t}))},saveGreeting:function(){var e=Object(u["a"])(regeneratorRuntime.mark((function e(t){var n=this;return regeneratorRuntime.wrap((function(e){while(1)switch(e.prev=e.next){case 0:return this.$refs.fieldset.disabled=!0,e.prev=1,e.next=4,window.contract.set_greeting({message:this.newGreeting});case 4:e.next=10;break;case 6:throw e.prev=6,e.t0=e["catch"](1),alert("Something went wrong! Maybe you need to sign out and back in? Check your browser console for more info."),e.t0;case 10:return e.prev=10,this.$refs.fieldset.disabled=!1,e.finish(10);case 13:this.savedGreeting=this.newGreeting,this.notificationVisible=!0,setTimeout((function(){n.notificationVisible=!1}),11e3);case 16:case"end":return e.stop()}}),e,this,[[1,6,10,13]])})));function t(t){return e.apply(this,arguments)}return t}(),logout:f}},U=E,A=Object(b["a"])(U,y,S,!1,null,null,null),G=A.exports,P=c()("testnet");console.log("networkId:".concat(P.networkId," CONTRACT_NAME:").concat(P.contractName)),window.networkId=P.networkId;var T={created:function(){document.title="fisha"},name:"App",components:{SignedOut:I,SignedIn:G},computed:{isSignedIn:function(){return window.walletConnection.isSignedIn()}}},R=T,M=Object(b["a"])(R,o,i,!1,null,null,null),V=M.exports;r["a"].config.productionTip=!1,window.nearInitPromise=g().then((function(){new r["a"]({render:function(e){return e(V)}}).$mount("#app")}))},db49:function(e,t){var n=Object({NODE_ENV:"production",BASE_URL:"/"}).VUE_APP_CONTRACT_NAME||"dev-1644621463460-56997790541024";function r(e){switch(e){case"production":case"mainnet":return{networkId:"mainnet",nodeUrl:"https://rpc.mainnet.near.org",contractName:n,walletUrl:"https://wallet.near.org",helperUrl:"https://helper.mainnet.near.org",explorerUrl:"https://explorer.mainnet.near.org"};case"development":case"testnet":return{networkId:"testnet",nodeUrl:"https://rpc.testnet.near.org",contractName:n,walletUrl:"https://wallet.testnet.near.org",helperUrl:"https://helper.testnet.near.org",explorerUrl:"https://explorer.testnet.near.org"};case"betanet":return{networkId:"betanet",nodeUrl:"https://rpc.betanet.near.org",contractName:n,walletUrl:"https://wallet.betanet.near.org",helperUrl:"https://helper.betanet.near.org",explorerUrl:"https://explorer.betanet.near.org"};case"local":return{networkId:"local",nodeUrl:"http://localhost:3030",keyPath:"".concat(Object({NODE_ENV:"production",BASE_URL:"/"}).HOME,"/.near/validator_key.json"),walletUrl:"http://localhost:4000/wallet",contractName:n};case"test":case"ci":return{networkId:"shared-test",nodeUrl:"https://rpc.ci-testnet.near.org",contractName:n,masterAccount:"test.near"};case"ci-betanet":return{networkId:"shared-test-staging",nodeUrl:"https://rpc.ci-betanet.near.org",contractName:n,masterAccount:"test.near"};default:throw Error("Unconfigured environment '".concat(e,"'. Can be configured in src/config.js."))}}e.exports=r}});
//# sourceMappingURL=app.4bb2c68d.js.map