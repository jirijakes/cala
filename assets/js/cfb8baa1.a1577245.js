"use strict";(self.webpackChunkwebsite=self.webpackChunkwebsite||[]).push([[563],{9705:(e,n,s)=>{s.r(n),s.d(n,{assets:()=>i,contentTitle:()=>o,default:()=>u,frontMatter:()=>a,metadata:()=>d,toc:()=>l});var c=s(4848),t=s(8453);const a={id:"list-accounts",title:"List accounts",slug:"/docs/demo/list-accounts"},o=void 0,d={id:"demo/list-accounts",title:"List accounts",description:"GraphQL body",source:"@site/docs/demo/list-accounts.mdx",sourceDirName:"demo",slug:"/docs/demo/list-accounts",permalink:"/docs/docs/demo/list-accounts",draft:!1,unlisted:!1,editUrl:"https://github.com/GaloyMoney/cala/edit/main/docs/demo/list-accounts.mdx",tags:[],version:"current",frontMatter:{id:"list-accounts",title:"List accounts",slug:"/docs/demo/list-accounts"}},i={},l=[{value:"GraphQL body",id:"graphql-body",level:3},{value:"Variables",id:"variables",level:3},{value:"Response",id:"response",level:3}];function r(e){const n={code:"code",h3:"h3",pre:"pre",...(0,t.R)(),...e.components};return(0,c.jsxs)(c.Fragment,{children:[(0,c.jsx)(n.h3,{id:"graphql-body",children:"GraphQL body"}),"\n",(0,c.jsx)(n.pre,{children:(0,c.jsx)(n.code,{className:"language-graphql",children:"query listAccounts() {\n  accounts(first: 10) {\n    nodes {\n      id\n      name\n    }\n  }\n}\n"})}),"\n",(0,c.jsx)(n.h3,{id:"variables",children:"Variables"}),"\n",(0,c.jsx)(n.pre,{children:(0,c.jsx)(n.code,{children:'{\n  "accountId": "3a7d421b-7f5a-43ca-ba6f-5f3e6ee67237",\n  "journalId": "bcc24f47-990c-457d-88cb-76332450ac77",\n  "currency": "USD"\n}\n'})}),"\n",(0,c.jsx)(n.h3,{id:"response",children:"Response"}),"\n",(0,c.jsx)(n.pre,{children:(0,c.jsx)(n.code,{className:"language-json",children:'{\n  "data": {\n    "account": {\n      "name": "Alice - Checking",\n      "balance": {\n        "settled": {\n          "normalBalance": {\n            "units": "9.53"\n          }\n        }\n      }\n    }\n  }\n}\n'})})]})}function u(e={}){const{wrapper:n}={...(0,t.R)(),...e.components};return n?(0,c.jsx)(n,{...e,children:(0,c.jsx)(r,{...e})}):r(e)}}}]);