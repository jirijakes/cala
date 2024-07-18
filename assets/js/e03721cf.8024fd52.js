"use strict";(self.webpackChunkwebsite=self.webpackChunkwebsite||[]).push([[6238,8401],{6104:(e,a,n)=>{n.r(a),n.d(a,{assets:()=>l,contentTitle:()=>o,default:()=>h,frontMatter:()=>c,metadata:()=>i,toc:()=>d});var t=n(4848),r=n(8453),s=n(7971);const c={id:"create-journal-and-accounts",title:"Create a Journal and Accounts",slug:"/docs/create-journal-and-accounts"},o=void 0,i={id:"demo/create-journal-and-accounts",title:"Create a Journal and Accounts",description:"Create different types of accounts (journal, checking, and debit accounts) through the GraphQL API.",source:"@site/docs/demo/create-journal-and-accounts.mdx",sourceDirName:"demo",slug:"/docs/create-journal-and-accounts",permalink:"/docs/create-journal-and-accounts",draft:!1,unlisted:!1,tags:[],version:"current",frontMatter:{id:"create-journal-and-accounts",title:"Create a Journal and Accounts",slug:"/docs/create-journal-and-accounts"},sidebar:"demoSidebar",previous:{title:"Try Cala",permalink:"/docs"},next:{title:"Create Transaction Templates",permalink:"/docs/tx-template-create"}},l={},d=[{value:"Create a Journal",id:"create-a-journal",level:2},{value:"Variables",id:"variables",level:3},{value:"GraphQL Request Body",id:"graphql-request-body",level:3},{value:"Response",id:"response",level:3},{value:"Create a Checking Account",id:"create-a-checking-account",level:2},{value:"Variables",id:"variables-1",level:3},{value:"GraphQL Request Body",id:"graphql-request-body-1",level:3},{value:"Response",id:"response-1",level:3},{value:"Create a Debit Account",id:"create-a-debit-account",level:2},{value:"Variables",id:"variables-2",level:3},{value:"GraphQL Request Body",id:"graphql-request-body-2",level:3},{value:"Response",id:"response-2",level:3}];function u(e){const a={code:"code",h2:"h2",h3:"h3",p:"p",...(0,r.R)(),...e.components};return(0,t.jsxs)(t.Fragment,{children:[(0,t.jsx)(a.p,{children:"Create different types of accounts (journal, checking, and debit accounts) through the GraphQL API."}),"\n",(0,t.jsx)(a.h2,{id:"create-a-journal",children:"Create a Journal"}),"\n",(0,t.jsx)(a.p,{children:'Initiate the creation of a journal, specifically a "General Ledger," which is fundamental in accounting for keeping track of all the financial transactions of a company.'}),"\n",(0,t.jsx)(a.h3,{id:"variables",children:"Variables"}),"\n",(0,t.jsx)(s.Ho,{variablesPath:"/gql/variables/journalCreate.json"}),"\n",(0,t.jsx)(a.p,{children:"The GraphQL mutation request input contains the unique ID and the name of the journal. The system creates a journal entity with these details."}),"\n",(0,t.jsx)(a.h3,{id:"graphql-request-body",children:"GraphQL Request Body"}),"\n",(0,t.jsx)(s._5,{queryPath:"/gql/journal-create.gql"}),"\n",(0,t.jsx)(a.h3,{id:"response",children:"Response"}),"\n",(0,t.jsx)(a.p,{children:"The system returns the details of the newly created journal, including\nits ID and name, confirming the successful creation."}),"\n",(0,t.jsx)(s.Dh,{responsePath:"/gql/responses/journalCreateResponse.json"}),"\n",(0,t.jsx)(a.h2,{id:"create-a-checking-account",children:"Create a Checking Account"}),"\n",(0,t.jsxs)(a.p,{children:["Next create a checking account named ",(0,t.jsx)(a.code,{children:"Alice - Checking"}),". Checking accounts are typically used for day-to-day banking transactions."]}),"\n",(0,t.jsxs)(a.p,{children:["The mutation request includes details like account ID, name, and other attributes like ",(0,t.jsx)(a.code,{children:"code"})," and ",(0,t.jsx)(a.code,{children:"normalBalanceType"}),". ",(0,t.jsx)("br",{}),"\nThe ",(0,t.jsx)(a.code,{children:"normalBalanceType"})," indicates whether the account is a debit or credit type, essential for correct accounting practices."]}),"\n",(0,t.jsx)(a.h3,{id:"variables-1",children:"Variables"}),"\n",(0,t.jsx)(s.Ho,{variablesPath:"/gql/variables/accountCreateChecking.json"}),"\n",(0,t.jsx)(a.h3,{id:"graphql-request-body-1",children:"GraphQL Request Body"}),"\n",(0,t.jsx)(s._5,{queryPath:"/gql/account-create.gql"}),"\n",(0,t.jsx)(a.h3,{id:"response-1",children:"Response"}),"\n",(0,t.jsx)(a.p,{children:"The response confirms the creation of the checking account with the specified details, ensuring the account is ready for transactional activities."}),"\n",(0,t.jsx)(s.Dh,{responsePath:"/gql/responses/accountCreateCheckingResponse.json"}),"\n",(0,t.jsx)(a.h2,{id:"create-a-debit-account",children:"Create a Debit Account"}),"\n",(0,t.jsxs)(a.p,{children:["Create a debit account labeled ",(0,t.jsx)(a.code,{children:"Assets"}),". Debit accounts are crucial for tracking resources owned by a business."]}),"\n",(0,t.jsxs)(a.p,{children:["Similar to the previous account creation, the mutation includes unique identifiers and account-specific information like the ",(0,t.jsx)(a.code,{children:"normalBalanceType"}),", which in this case is ",(0,t.jsx)(a.code,{children:"DEBIT"}),", indicating the nature of transactions expected."]}),"\n",(0,t.jsx)(a.h3,{id:"variables-2",children:"Variables"}),"\n",(0,t.jsx)(s.Ho,{variablesPath:"/gql/variables/accountCreateDebit.json"}),"\n",(0,t.jsx)(a.h3,{id:"graphql-request-body-2",children:"GraphQL Request Body"}),"\n",(0,t.jsx)(s._5,{queryPath:"/gql/account-create.gql"}),"\n",(0,t.jsx)(a.h3,{id:"response-2",children:"Response"}),"\n",(0,t.jsx)(a.p,{children:"The response shows the successful creation of the debit account, confirming that\nit's set up to track asset-related transactions."}),"\n",(0,t.jsx)(s.Dh,{responsePath:"/gql/responses/accountCreateDebitResponse.json"})]})}function h(e={}){const{wrapper:a}={...(0,r.R)(),...e.components};return a?(0,t.jsx)(a,{...e,children:(0,t.jsx)(u,{...e})}):u(e)}},7971:(e,a,n)=>{n.d(a,{Dh:()=>i,Ho:()=>o,_5:()=>c});var t=n(6540),r=n(4110),s=n(4848);const c=e=>{let{queryPath:a}=e;const[n,c]=(0,t.useState)("");return(0,t.useEffect)((()=>{(async()=>{try{const e=await fetch(a),n=await e.text();c(n)}catch(e){console.error("Error loading query:",e)}})()}),[a]),(0,s.jsx)(r.A,{className:"language-graphql",children:n})},o=e=>{let{variablesPath:a}=e;const[n,c]=(0,t.useState)({});return(0,t.useEffect)((()=>{(async()=>{try{const e=await fetch(a),n=await e.json();c(n)}catch(e){console.error("Error loading variables:",e)}})()}),[a]),(0,s.jsx)(r.A,{className:"language-json",children:JSON.stringify(n,null,2)})},i=e=>{let{responsePath:a}=e;const[n,c]=(0,t.useState)({});return(0,t.useEffect)((()=>{(async()=>{try{const e=await fetch(a),n=await e.json();c(n)}catch(e){console.error("Error loading response:",e)}})()}),[a]),(0,s.jsx)(r.A,{className:"language-json",children:JSON.stringify(n,null,2)})}},1433:(e,a,n)=>{n.d(a,{A:()=>s});var t=n(6540),r=n(4848);const s={React:t,...t,ButtonExample:e=>(0,r.jsx)("button",{...e,style:{backgroundColor:"white",color:"black",border:"solid red",borderRadius:20,padding:10,cursor:"pointer",...e.style}}),handleAuthenticatedRequest:async function(e,a,n,t){if(void 0===t&&(t={}),!e)throw new Error("Not authenticated");if(!n)throw new Error("No GraphQL query provided");try{const r=await fetch(a,{method:"POST",headers:{"Content-Type":"application/json",Accept:"application/json","X-API-KEY":`${e}`},body:JSON.stringify({query:n,variables:t})});if(!r.ok){const e=await r.text();throw new Error(`Error response from server: ${e}`)}const s=r.headers.get("content-type");if(s&&s.includes("application/json")){return await r.json()}throw new Error(`Unexpected content type: ${s}`)}catch(r){throw console.error("There was an error making the authenticated request:",r),r}}}}}]);