# ```arti_client``` example

This repo demonstrates how to get the content of a hidden service via tor using plain HTTP.

More examples are available [here](https://gitlab.torproject.org/tpo/core/arti/-/tree/main/crates/arti-client/examples).

When using contacting port 443, you'll probably get a ```400 Bad Request```, because it can only make HTTP requests.
If you figure out how to use it with HTTPS, please make a pull request (me stoopid😅)

---

NOTE: arti_client is not yet stable. Bear this in mind when using it in a production environment. Also, because it is in the early stages of development, the API will probably change significantly in the future.
