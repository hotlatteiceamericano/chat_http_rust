# Chat HTTP Server

## Account Creation & Auth
As this project will be a small-scale side project, I want to implement the account creation and login in the same handler.

It will send out a short-lived OTP to the email address client provides

Later when user provides an matched OTP, HTTP server will create account if no such email exist, and then proceed with normal account login flow. 

This REST API will be implemented with a HTTP POST method, expecting the email in its payload.

It will store user account information using a document db, choosing MongoDB as the storage and MongoDB Atlas as the hosting platform because of its generous free tier.

## Verify
Expect user to input the OTP sent to user's email address. Provide WebSocker server URL and a valid JWT token upon seccuessful varification. Reutrn 403 when OTP is not matched.

## WebSocket Server Discovery
WebSocket server for this project is designed to be as state-less as possible, other than establishing websocket connection. HTTP server has will be querying from a databased to find out which websocket server is available for connection.

## Conversation List
Take an user id as argument, provide a list of conversation who previously had chatted with this user id. Should be returning list of User (currently lived under ui repo, needs to put common struct such as User, Message to a common place for ui, websocket and http repo to share).

## Chat history
Take sender user id and recier user id as the arguments, provide the chat history between the two.

## Note
Note that all the functionality listed above need to have valid JWT token presented in the http Authorization header.

