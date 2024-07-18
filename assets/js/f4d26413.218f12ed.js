"use strict";(self.webpackChunkwebsite=self.webpackChunkwebsite||[]).push([[6413,8401],{9143:(e,t,n)=>{n.r(t),n.d(t,{assets:()=>l,contentTitle:()=>c,default:()=>u,frontMatter:()=>r,metadata:()=>o,toc:()=>d});var s=n(4848),a=n(8453),i=n(7971);const r={id:"tx-template-create",title:"Create Transaction Templates",slug:"/docs/tx-template-create"},c=void 0,o={id:"demo/tx-template-create",title:"Create Transaction Templates",description:"This functionality allows a user (an administrator or financial manager) to define templates for recurring transaction types - specifically deposits and withdrawals. By defining these templates, the user ensures consistency, accuracy, and efficiency in transaction processing.",source:"@site/docs/demo/tx-template-create.mdx",sourceDirName:"demo",slug:"/docs/tx-template-create",permalink:"/docs/tx-template-create",draft:!1,unlisted:!1,tags:[],version:"current",frontMatter:{id:"tx-template-create",title:"Create Transaction Templates",slug:"/docs/tx-template-create"},sidebar:"demoSidebar",previous:{title:"Create a Journal and Accounts",permalink:"/docs/create-journal-and-accounts"},next:{title:"Post a Transaction",permalink:"/docs/transaction-post"}},l={},d=[{value:"Process",id:"process",level:2},{value:"Deposit Transaction Template",id:"deposit-transaction-template",level:3},{value:"Withdrawal Transaction Template",id:"withdrawal-transaction-template",level:3},{value:"GraphQL Request Body",id:"graphql-request-body",level:3},{value:"Variables",id:"variables",level:3},{value:"Response",id:"response",level:3},{value:"Significance",id:"significance",level:2}];function h(e){const t={code:"code",h2:"h2",h3:"h3",li:"li",p:"p",strong:"strong",ul:"ul",...(0,a.R)(),...e.components};return(0,s.jsxs)(s.Fragment,{children:[(0,s.jsx)(t.p,{children:"This functionality allows a user (an administrator or financial manager) to define templates for recurring transaction types - specifically deposits and withdrawals. By defining these templates, the user ensures consistency, accuracy, and efficiency in transaction processing."}),"\n",(0,s.jsx)(t.h2,{id:"process",children:"Process"}),"\n",(0,s.jsx)(t.h3,{id:"deposit-transaction-template",children:"Deposit Transaction Template"}),"\n",(0,s.jsxs)(t.ul,{children:["\n",(0,s.jsxs)(t.li,{children:["\n",(0,s.jsxs)(t.p,{children:[(0,s.jsx)(t.strong,{children:"Identification and Description"}),": The template is uniquely identified by a ",(0,s.jsx)(t.code,{children:"txTemplateId"})," and a descriptive code. The description explains that this template is for an ACH credit into a customer's account."]}),"\n"]}),"\n",(0,s.jsxs)(t.li,{children:["\n",(0,s.jsxs)(t.p,{children:[(0,s.jsx)(t.strong,{children:"Parameters"}),": Parameters like the account ID (",(0,s.jsx)(t.code,{children:"account"}),"), transaction amount (",(0,s.jsx)(t.code,{children:"amount"}),"), and effective date (",(0,s.jsx)(t.code,{children:"effective"}),") are defined, which are necessary inputs for executing a deposit transaction."]}),"\n"]}),"\n",(0,s.jsxs)(t.li,{children:["\n",(0,s.jsxs)(t.p,{children:[(0,s.jsx)(t.strong,{children:"Transaction and Entries Definition"}),":\nThe transaction input specifies which journal to log this transaction under and when it becomes effective. Entries detail the movement of funds, specifying which account to debit and which to credit, in what amount, and under what transaction conditions, e.g., ",(0,s.jsx)(t.code,{children:"currency"})," and transaction type (",(0,s.jsx)(t.code,{children:"entryType"}),")."]}),"\n"]}),"\n"]}),"\n",(0,s.jsx)(t.h3,{id:"withdrawal-transaction-template",children:"Withdrawal Transaction Template"}),"\n",(0,s.jsxs)(t.ul,{children:["\n",(0,s.jsxs)(t.li,{children:["\n",(0,s.jsxs)(t.p,{children:[(0,s.jsx)(t.strong,{children:"Identification and Description"}),": Similarly, this template has its unique identifier and code, and is described for use in ACH debits from a customer's account."]}),"\n"]}),"\n",(0,s.jsxs)(t.li,{children:["\n",(0,s.jsxs)(t.p,{children:[(0,s.jsx)(t.strong,{children:"Parameters"}),": It uses the same types of parameters as the deposit template, allowing for consistency across templates."]}),"\n"]}),"\n",(0,s.jsxs)(t.li,{children:["\n",(0,s.jsxs)(t.p,{children:[(0,s.jsx)(t.strong,{children:"Transaction and Entries Definition"}),": This template defines how funds will be withdrawn, including debiting the customer's account and crediting an asset account, with specifics on the transaction type and conditions."]}),"\n"]}),"\n"]}),"\n",(0,s.jsx)(t.h3,{id:"graphql-request-body",children:"GraphQL Request Body"}),"\n",(0,s.jsx)(i._5,{queryPath:"/gql/tx-template-create.gql"}),"\n",(0,s.jsx)(t.h3,{id:"variables",children:"Variables"}),"\n",(0,s.jsxs)(t.ul,{children:["\n",(0,s.jsxs)(t.li,{children:[(0,s.jsx)(t.strong,{children:"depositTemplateId"}),": A unique identifier for the deposit transaction template. This ensures that the template is uniquely recognized within the system."]}),"\n",(0,s.jsxs)(t.li,{children:[(0,s.jsx)(t.strong,{children:"depositTemplateCode"}),": A code that describes the deposit transaction template. This provides an easy reference for users."]}),"\n",(0,s.jsxs)(t.li,{children:[(0,s.jsx)(t.strong,{children:"withdrawalTemplateId"}),": A unique identifier for the withdrawal transaction template. This ensures that the template is uniquely recognized within the system."]}),"\n",(0,s.jsxs)(t.li,{children:[(0,s.jsx)(t.strong,{children:"withdrawalTemplateCode"}),": A code that describes the withdrawal transaction template. This provides an easy reference for users."]}),"\n",(0,s.jsxs)(t.li,{children:[(0,s.jsx)(t.strong,{children:"journalId"}),": The identifier for the journal where the transactions will be logged. This links the transaction to the correct financial journal."]}),"\n",(0,s.jsxs)(t.li,{children:[(0,s.jsx)(t.strong,{children:"assetAccountId"}),": The identifier for the asset account involved in the transactions. This specifies which account will be debited or credited during the transaction."]}),"\n"]}),"\n",(0,s.jsx)(i.Ho,{variablesPath:"/gql/variables/txTemplateCreate.json"}),"\n",(0,s.jsx)(t.h3,{id:"response",children:"Response"}),"\n",(0,s.jsx)(t.p,{children:"Upon successful submission of the GraphQL mutations, the system creates these templates and returns their IDs in the response. This confirms that the templates are ready for use in future transactions."}),"\n",(0,s.jsx)(i.Dh,{responsePath:"/gql/responses/txTemplateCreateResponse.json"}),"\n",(0,s.jsx)(t.h2,{id:"significance",children:"Significance"}),"\n",(0,s.jsx)(t.p,{children:"Creating transaction templates standardizes the execution of frequent transaction types, reducing errors and the time needed for transaction entry. This systematization is crucial for large organizations or financial institutions where transactions occur regularly and need to adhere to strict regulatory and internal standards. The use of GraphQL allows for clear, customizable, and direct communication with the database, facilitating dynamic interaction with the financial data."})]})}function u(e={}){const{wrapper:t}={...(0,a.R)(),...e.components};return t?(0,s.jsx)(t,{...e,children:(0,s.jsx)(h,{...e})}):h(e)}},7971:(e,t,n)=>{n.d(t,{Dh:()=>o,Ho:()=>c,_5:()=>r});var s=n(6540),a=n(4110),i=n(4848);const r=e=>{let{queryPath:t}=e;const[n,r]=(0,s.useState)("");return(0,s.useEffect)((()=>{(async()=>{try{const e=await fetch(t),n=await e.text();r(n)}catch(e){console.error("Error loading query:",e)}})()}),[t]),(0,i.jsx)(a.A,{className:"language-graphql",children:n})},c=e=>{let{variablesPath:t}=e;const[n,r]=(0,s.useState)({});return(0,s.useEffect)((()=>{(async()=>{try{const e=await fetch(t),n=await e.json();r(n)}catch(e){console.error("Error loading variables:",e)}})()}),[t]),(0,i.jsx)(a.A,{className:"language-json",children:JSON.stringify(n,null,2)})},o=e=>{let{responsePath:t}=e;const[n,r]=(0,s.useState)({});return(0,s.useEffect)((()=>{(async()=>{try{const e=await fetch(t),n=await e.json();r(n)}catch(e){console.error("Error loading response:",e)}})()}),[t]),(0,i.jsx)(a.A,{className:"language-json",children:JSON.stringify(n,null,2)})}},1433:(e,t,n)=>{n.d(t,{A:()=>i});var s=n(6540),a=n(4848);const i={React:s,...s,ButtonExample:e=>(0,a.jsx)("button",{...e,style:{backgroundColor:"white",color:"black",border:"solid red",borderRadius:20,padding:10,cursor:"pointer",...e.style}}),handleAuthenticatedRequest:async function(e,t,n,s){if(void 0===s&&(s={}),!e)throw new Error("Not authenticated");if(!n)throw new Error("No GraphQL query provided");try{const a=await fetch(t,{method:"POST",headers:{"Content-Type":"application/json",Accept:"application/json","X-API-KEY":`${e}`},body:JSON.stringify({query:n,variables:s})});if(!a.ok){const e=await a.text();throw new Error(`Error response from server: ${e}`)}const i=a.headers.get("content-type");if(i&&i.includes("application/json")){return await a.json()}throw new Error(`Unexpected content type: ${i}`)}catch(a){throw console.error("There was an error making the authenticated request:",a),a}}}}}]);