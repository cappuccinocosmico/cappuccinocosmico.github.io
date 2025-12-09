+++
title = "Juristictions"
date = 2025-01-01

[extra]
author = "Nicole Venner"
+++

---
title: What Does the Data of our app look like Post Single State and post PUC.
author: Nic Venner
---


# Our current pipeline cant handle most goverment data but can handle a lot more then state PUC's 

So 



Step 1: 
Agent w/ selinum ->
Go to website and figure out what data exists
-> Return completed python schema 

Step 2:
Agent w/ selenium & Scrapegraph AI -> 
Go to specific subwebsite for a specific schema (example: documents from specific filing page)
-> Output python selenium code that extracts the data 

Concurrent Step 3:
Regular Reasoning LLM -> 
Take Schemas and write adapters to generic code 
-> Generic Adapters 

Concurrent Step 3:
Regular Reasoning LLM -> 
Take scrapers and refactor it so that it saves data as an intermediate ->
Regularized Scrapers 


Step 4: 
Regular Reasoning LLM ->
Combine Schemas and Refactored Scrapers into final class.







