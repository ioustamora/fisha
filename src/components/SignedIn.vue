<template>
  <main>
    <div class="row">
      <div class="col">
        <ul class="list-group">
          <li class="list-group-item d-flex justify-content-between align-items-center">
            Nemo
            <span class="badge bg-primary rounded-pill">1</span>
          </li>
          <li class="list-group-item d-flex justify-content-between align-items-center">
            Goldie
            <span class="badge bg-primary rounded-pill">0</span>
          </li>
          <li class="list-group-item d-flex justify-content-between align-items-center">
            Bubbles
            <span class="badge bg-primary rounded-pill">0</span>
          </li>
          <li class="list-group-item d-flex justify-content-between align-items-center">
            Dory
            <span class="badge bg-primary rounded-pill">0</span>
          </li>
        </ul>
      </div>
      <div class="col">
        <div class="card text-white bg-secondary mb-3" style="max-width: 20rem;">
          <div class="card-header">Caviar</div>
          <div class="card-body">
            <div>
      <h1>
        <label
          for="greeting"
          style="color: var(--secondary);border-bottom: 2px solid var(--secondary);"
        >{{ savedGreeting }}</label>
        {{ accountId }}
      </h1>
      <form v-on:submit.prevent="saveGreeting">
        <fieldset ref="fieldset">
          <label
            for="greeting"
            style="display:block; color:var(--gray);margin-bottom:0.5em;"
          >Change greeting</label>
          <div style="display:flex">
            <input v-model="newGreeting" autocomplete="off" id="greeting" style="flex:1" />
            <button id="save" style="border-radius:0 5px 5px 0">Save</button>
          </div>
        </fieldset>
      </form>
      <p>Look at that! A Hello World app! This greeting is stored on the NEAR blockchain. Check it out:</p>
      <hr />
      <p>
        If you have questions or issues - feel free to contact as by email: support@fisha.co.in
      </p>
    </div>
          </div>
        </div>
      </div>
      <div class="col">
        <div class="card text-white bg-secondary mb-3" style="max-width: 20rem;">
          <div class="card-header">Wallet Data</div>
          <div class="card-body">
            <p class="card-text">
              some data
            </p>
          </div>
        </div>
        <div class="card text-white bg-secondary mb-3" style="max-width: 20rem;">
          <div class="card-header">Dev Notes</div>
          <div class="card-body">
            <p class="card-text">
              Some dev notes) Lock 1 near token to start game.
            </p>
          </div>
        </div>
      </div>
    </div>
    

    <Notification
      v-show="notificationVisible"
      ref="notification"
      :networkId="networkId"
      :msg="'called method: set_greeting'"
      :contractId="contractId"
      :visible="false"
    />
  </main>
</template>

<script>
import { utils } from 'near-api-js'
import { logout } from "../utils"

import Notification from "./Notification.vue"

export default {
  name: "SignedIn",

  beforeMount() {
    if (this.isSignedIn) {
      this.retrieveSavedGreeting()
    }
  },

  components: {
    Notification,
  },

  data: function () {
    return {
      savedGreeting: "",
      newGreeting: "",
      notificationVisible: false,
    }
  },

  computed: {
    isSignedIn() {
      return window.walletConnection? window.walletConnection.isSignedIn(): false
    },
    accountId() {
      return window.accountId
    },
    contractId() {
      return window.contract? window.contract.contractId: null
    },
    networkId() {
      return window.networkId
    },
  },

  created() {
    
  },

  methods: {
    retrieveSavedGreeting() {
      //retrieve greeting
      window.contract
        .get_greeting({ account_id: window.accountId })
        .then((greetingFromContract) => {
          this.savedGreeting = greetingFromContract
          this.newGreeting = greetingFromContract
        })
    },

    saveGreeting: async function (event) {
      // fired on form submit button used to update the greeting

      // disable the form while the value gets updated on-chain
      this.$refs.fieldset.disabled = true

      try {
        
        // make an update call to the smart contract
        await window.contract.set_greeting({
          // pass the new greeting
          message: this.newGreeting,
        })
      } catch (e) {
        alert(
          "Something went wrong! " +
            "Maybe you need to sign out and back in? " +
            "Check your browser console for more info."
        )
        throw e //re-throw
      } finally {
        // re-enable the form, whether the call succeeded or failed
        this.$refs.fieldset.disabled = false
      }

      // update savedGreeting with persisted value
      this.savedGreeting = this.newGreeting

      this.notificationVisible = true //show new notification

      // remove Notification again after css animation completes
      // this allows it to be shown again next time the form is submitted
      setTimeout(() => {
        this.notificationVisible = false
      }, 11000)

    },

    logout: logout,
  },
}
</script>
