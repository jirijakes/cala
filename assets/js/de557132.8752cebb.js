"use strict";(self.webpackChunkwebsite=self.webpackChunkwebsite||[]).push([[9799,8401],{1034:(e,n,a)=>{a.r(n),a.d(n,{assets:()=>l,contentTitle:()=>r,default:()=>u,frontMatter:()=>i,metadata:()=>o,toc:()=>d});var c=a(4848),t=a(8453),s=a(7971);const i={id:"check-account-balance",title:"Check the Balance of an Account",slug:"/docs/check-account-balance"},r=void 0,o={id:"demo/check-account-balance",title:"Check the Balance of an Account",description:"The functionality is essential for users (e.g., account holders, financial managers) to view the balance of a specific account in a particular journal and currency. This allows for real-time financial monitoring and decision-making based on up-to-date account information.",source:"@site/docs/demo/check-account-balance.mdx",sourceDirName:"demo",slug:"/docs/check-account-balance",permalink:"/docs/check-account-balance",draft:!1,unlisted:!1,tags:[],version:"current",frontMatter:{id:"check-account-balance",title:"Check the Balance of an Account",slug:"/docs/check-account-balance"},sidebar:"demoSidebar",previous:{title:"Post a Transaction",permalink:"/docs/transaction-post"},next:{title:"Use an Account Set",permalink:"/docs/account-set"}},l={},d=[{value:"Process",id:"process",level:2},{value:"Variables",id:"variables",level:3},{value:"GraphQL Request Body",id:"graphql-request-body",level:3},{value:"Response",id:"response",level:3},{value:"Significance",id:"significance",level:2}];function h(e){const n={code:"code",h2:"h2",h3:"h3",li:"li",p:"p",strong:"strong",ul:"ul",...(0,t.R)(),...e.components};return(0,c.jsxs)(c.Fragment,{children:[(0,c.jsx)(n.p,{children:"The functionality is essential for users (e.g., account holders, financial managers) to view the balance of a specific account in a particular journal and currency. This allows for real-time financial monitoring and decision-making based on up-to-date account information."}),"\n",(0,c.jsx)(n.h2,{id:"process",children:"Process"}),"\n",(0,c.jsx)(n.h3,{id:"variables",children:"Variables"}),"\n",(0,c.jsxs)(n.ul,{children:["\n",(0,c.jsxs)(n.li,{children:[(0,c.jsx)(n.strong,{children:"Account ID"}),": The ",(0,c.jsx)(n.code,{children:"accountId"})," uniquely identifies the account whose balance is being queried. This ensures that the query is precise and retrieves information for the correct account."]}),"\n",(0,c.jsxs)(n.li,{children:[(0,c.jsx)(n.strong,{children:"Journal ID"}),": The ",(0,c.jsx)(n.code,{children:"journalId"})," specifies which journal to check for the account's balance. This is important because an account may have different balances in different journals due to various types of transactions."]}),"\n",(0,c.jsxs)(n.li,{children:[(0,c.jsx)(n.strong,{children:"Currency"}),": The ",(0,c.jsx)(n.code,{children:"currency"})," parameter ensures that the balance is provided in the desired currency, in this case, USD. This is crucial for accuracy and relevance, especially in multi-currency environments."]}),"\n"]}),"\n",(0,c.jsx)(s.Ho,{variablesPath:"/gql/variables/accountWithBalance.json"}),"\n",(0,c.jsx)(n.h3,{id:"graphql-request-body",children:"GraphQL Request Body"}),"\n",(0,c.jsxs)(n.p,{children:["The ",(0,c.jsx)(n.code,{children:"accountWithBalance"})," query is executed with the provided inputs. The query fetches the account's name and its settled balance in the specified journal and currency."]}),"\n",(0,c.jsx)(s._5,{queryPath:"/gql/account-with-balance.gql"}),"\n",(0,c.jsx)(n.p,{children:"The system retrieves the settled balance from the specified journal for the given account."}),"\n",(0,c.jsx)(n.h3,{id:"response",children:"Response"}),"\n",(0,c.jsx)(n.p,{children:"The response includes the account's name and its settled balance in the specified currency and journal. This information is returned in a structured JSON format, which includes:"}),"\n",(0,c.jsxs)(n.ul,{children:["\n",(0,c.jsxs)(n.li,{children:[(0,c.jsx)(n.strong,{children:"Account Name"}),': "Alice - Checking", confirming that the balance belongs to the correct account.']}),"\n",(0,c.jsxs)(n.li,{children:[(0,c.jsx)(n.strong,{children:"Settled Balance"}),": The ",(0,c.jsx)(n.code,{children:"normalBalance"})," ",(0,c.jsx)(n.code,{children:"units"}),' show the account\'s balance as "9.53" USD, indicating the available settled funds in the account.']}),"\n"]}),"\n",(0,c.jsx)(s.Dh,{responsePath:"/gql/responses/accountWithBalanceResponse.json"}),"\n",(0,c.jsx)(n.h2,{id:"significance",children:"Significance"}),"\n",(0,c.jsx)(n.p,{children:"Checking account balances is a fundamental operation in financial management. It allows users to:"}),"\n",(0,c.jsxs)(n.ul,{children:["\n",(0,c.jsxs)(n.li,{children:[(0,c.jsx)(n.strong,{children:"Monitor Financial Status"}),": Users can keep track of their available funds, ensuring they are aware of their financial position."]}),"\n",(0,c.jsxs)(n.li,{children:[(0,c.jsx)(n.strong,{children:"Make Informed Decisions"}),": Accurate and up-to-date balance information is essential for making financial decisions, such as initiating transactions, budgeting, or investing."]}),"\n",(0,c.jsxs)(n.li,{children:[(0,c.jsx)(n.strong,{children:"Ensure Compliance and Accuracy"}),": Regularly checking balances helps in identifying any discrepancies or issues early, maintaining the integrity of financial records."]}),"\n"]})]})}function u(e={}){const{wrapper:n}={...(0,t.R)(),...e.components};return n?(0,c.jsx)(n,{...e,children:(0,c.jsx)(h,{...e})}):h(e)}},7971:(e,n,a)=>{a.d(n,{Dh:()=>o,Ho:()=>r,_5:()=>i});var c=a(6540),t=a(4110),s=a(4848);const i=e=>{let{queryPath:n}=e;const[a,i]=(0,c.useState)("");return(0,c.useEffect)((()=>{(async()=>{try{const e=await fetch(n),a=await e.text();i(a)}catch(e){console.error("Error loading query:",e)}})()}),[n]),(0,s.jsx)(t.A,{className:"language-graphql",children:a})},r=e=>{let{variablesPath:n}=e;const[a,i]=(0,c.useState)({});return(0,c.useEffect)((()=>{(async()=>{try{const e=await fetch(n),a=await e.json();i(a)}catch(e){console.error("Error loading variables:",e)}})()}),[n]),(0,s.jsx)(t.A,{className:"language-json",children:JSON.stringify(a,null,2)})},o=e=>{let{responsePath:n}=e;const[a,i]=(0,c.useState)({});return(0,c.useEffect)((()=>{(async()=>{try{const e=await fetch(n),a=await e.json();i(a)}catch(e){console.error("Error loading response:",e)}})()}),[n]),(0,s.jsx)(t.A,{className:"language-json",children:JSON.stringify(a,null,2)})}},1433:(e,n,a)=>{a.d(n,{A:()=>s});var c=a(6540),t=a(4848);const s={React:c,...c,ButtonExample:e=>(0,t.jsx)("button",{...e,style:{backgroundColor:"white",color:"black",border:"solid red",borderRadius:20,padding:10,cursor:"pointer",...e.style}}),handleAuthenticatedRequest:async function(e,n,a,c){if(void 0===c&&(c={}),!e)throw new Error("Not authenticated");if(!a)throw new Error("No GraphQL query provided");try{const t=await fetch(n,{method:"POST",headers:{"Content-Type":"application/json",Accept:"application/json","X-API-KEY":`${e}`},body:JSON.stringify({query:a,variables:c})});if(!t.ok){const e=await t.text();throw new Error(`Error response from server: ${e}`)}const s=t.headers.get("content-type");if(s&&s.includes("application/json")){return await t.json()}throw new Error(`Unexpected content type: ${s}`)}catch(t){throw console.error("There was an error making the authenticated request:",t),t}}}}}]);