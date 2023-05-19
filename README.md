# bot-matrix-sdk
Using the <a href="https://matrix-org.github.io/matrix-rust-sdk/matrix_sdk/index.html">matrix-sdk</a> for Rust to develop a bot that provides the latest news about the Kusama community.

## Usage
To implement this code for any Matrix user you own, follow these steps:
<ol>
  <li> Set your password as an environment variable in a file named .env placed in the main directory.</li>
  <li> Modify line 52 of main.rs to include your own username and homeserver.</li>
  <li>  Once you have made these changes, navigate to the src directory and run <code>cargo run</code>.</li>
</ol>

## Current features
As of 05-19-23, this code allows you to sync your user and view the messages you send and receive through your terminal. Additionally, your user can automatically join any public room it is invited to.

## Upcoming features
In the coming days, I will be adding new actions to the code, including:
<ul>
  <li> Commands where the bot can respond to various queries.</li>
  <li> The ability to fetch and update the latest news from the Kusama community and governance. This feature can be configured to update once a day, once a week, or on demand (specific timing is yet to be defined).</li>
</ul>
