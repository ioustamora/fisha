<template>

  <main>
   <div class="row">
  <hr/>
</div>
<div class="row">
  <div class="col">
    <div class="card text-white bg-primary mb-3" style="max-width: 20rem;">
  <div class="card-header">HODL &amp; Start -> </div>
  <div class="card-body">
    <p class="card-text">Lock 1 near to start game and achive game assets starter pack. You can withdraw 1 near after unlock.</p>
  </div>
</div>
  </div>
  <div class="col">
    <div class="card text-white bg-warning mb-3" style="max-width: 20rem;">
  <div class="card-header">Learn DeFi -> </div>
  <div class="card-body">
    <p class="card-text">Try all our gamefi ideas to get more assets. Use lottery, swap, staking and more upcoming fitures. </p>
  </div>
</div>
  </div>
  <div class="col">
    <div class="card text-white bg-success mb-3" style="max-width: 20rem;">
  <div class="card-header">Have Fun -></div>
  <div class="card-body">
    <p class="card-text">You can't lose in this game. Ther is onli Win!) After unlock you can get 1 near back. Plus FISHA bonuses)</p>
  </div>
</div>

  </div>
  <div class="col">
    <div class="card text-white bg-info mb-3" style="max-width: 20rem;">
  <div class="card-header">Take Profit!</div>
  <div class="card-body">
    <p class="card-text">Get NFT's. Stake caviar and exchange other in-game assets to get FISHA tokens! Plus HODL 1 NEAR)</p>
  </div>
</div>
  </div>
</div>

<div class="row">
  <hr/>
</div>

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
