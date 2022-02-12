<template>
  <main>
    <h1>Welcome to FISHA!</h1>
    <p>
      FISHA in development and works in NEAR testnet chain. To start app you need to SignIn. The button
      below will sign you in using NEAR testnet Wallet (wallet.testnet.near.org). This works just like the main
      network ("mainnet") wallet, but the NEAR Tokens on testnet aren't convertible to other currencies - they're just for testing! 
    </p>
    <p>Go ahead and click the button below to try it out:</p>
    <p style="text-align:center; margin-top:2.5em">
      <button v-on:click="login">Sign in</button>
    </p>
  </main>
</template>

<script>
import { login } from "../utils"

export default {
  name: "SignedOut",

  methods: {
    login() {
      console.log("calling utils.login")
      login()
    },
  },
}
</script>
