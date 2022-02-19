<template>
  <main>
    <div class="row">
      <div class="col">
        <div class="card text-white bg-secondary mb-3" style="max-width: 20rem;">
          <div class="card-header">
            Aquarium 
            <div class="float-end">
              <button class="btn btn-outline-success btn-sm" v-on:click="harvest">
                Harvest
              </button>
            </div>
          </div>
          <div class="card-body" style="padding:0.5rem;">
            <ul class="list-group">
          <li class="list-group-item d-flex justify-content-between align-items-center">
            Nemo
            <span class="badge bg-primary rounded-pill">{{ nemo }}</span>
          </li>
          <li class="list-group-item d-flex justify-content-between align-items-center">
            Dori
            <span class="badge bg-primary rounded-pill">{{ dori }}</span>
          </li>
          <li class="list-group-item d-flex justify-content-between align-items-center">
            Captain
            <span class="badge bg-primary rounded-pill">{{ captain }}</span>
          </li>
          <li class="list-group-item d-flex justify-content-between align-items-center">
            Ariel
            <span class="badge bg-primary rounded-pill">{{ ariel }}</span>
          </li>
        </ul>
          </div>
        </div>
      </div>
      <div class="col">
        <div class="card text-white bg-secondary mb-3" style="max-width: 20rem;">
          <div class="card-header">
            Vault 
            <div class="float-end">
              <button class="btn btn-outline-success btn-sm" v-on:click="harvest_stake">
                Harvest
              </button>
            </div>
          </div>
          <div class="card-body">
            <div>
              <p>Available: {{ caviar }} CAVIAR</p>
              <hr/>
              <form>
                <fieldset>
                  <div class="form-group">
                      <div class="input-group input-group-sm mb-3">
                        <button class="btn btn-info btn-sm" type="button" id="button-addon1" v-on:click="stakeMinus()">-</button>
                        <input type="text" class="form-control" placeholder="0" aria-label="0" aria-describedby="button-addon5" v-model="stakeInput.val">
                        <button class="btn btn-info btn-sm" type="button" id="button-addon2" v-on:click="stakePlus()">+</button>
                        <button class="btn btn-secondary btn-sm" type="button" id="button-addon3" v-on:click="stakeMax()">MAX</button>
                        <button class="btn btn-warning btn-sm" id="button-addon6" v-on:click.prevent="stakeCaviar()">Stake</button>
                      </div>
                    </div>
                </fieldset>
              </form>
              <p>Staked: {{ caviarVault }} CAVIAR </p>
              <button class="btn btn-outline-primary btn-sm" id="button-addon6" v-show="caviarVault >= 100" v-on:click="unstakeCaviar()">Unstake</button>
            </div>
          </div>
        </div>
      </div>
      <div class="col">
        <div class="card text-white bg-secondary mb-3" style="max-width: 20rem;">
          <div class="card-header">Swap Assets</div>
          <div class="card-body" style="padding:0.5rem;">
            <ul class="list-group">
              <li class="list-group-item d-flex justify-content-between align-items-center">
                10 Nemo -> 1 Dori
                <button class="btn btn-success btn-sm disabled" id="button11">swap</button>
              </li>
              <li class="list-group-item d-flex justify-content-between align-items-center">
                10 Dori -> 1 Captain
                <button class="btn btn-success btn-sm disabled" id="button12">swap</button>
              </li>
              <li class="list-group-item d-flex justify-content-between align-items-center">
                10 Captain -> 1 Ariel
                <button class="btn btn-success btn-sm disabled" id="button13">swap</button>
              </li>
            </ul>
          </div>
        </div>
      </div>
      <div class="col">
        <div class="card text-white bg-secondary mb-3" style="max-width: 20rem;">
          <div class="card-header">Exchange CAVIAR & FISHA</div>
          <div class="card-body">
            <p class="card-text">
              coming soon
            </p>
          </div>
        </div>
        <div class="card text-white bg-secondary mb-3" style="max-width: 20rem;">
          <div class="card-header">Stake FISHA</div>
          <div class="card-body">
            <p class="card-text">
              coming soon
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
      this.updateState()
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
      caviar: 0,
      caviarVault: 0,
      nemo: 0,
      dori: 0,
      captain: 0,
      ariel: 0,
      stakeInput: {
        val: 0,
      },
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
    updateState() {
      window.contract.get_caviar({ account_id: window.accountId })
        .then((caviar) => {
          this.caviar = caviar
        })
    window.contract.get_nemo({ account_id: window.accountId })
        .then((nemo) => {
          this.nemo = nemo
        })
    window.contract.get_caviar_vault({ account_id: window.accountId })
        .then((vault) => {
          this.caviarVault = vault
        })
    },
    harvest() {
      window.contract
        .harvest_fish({ account_id: window.accountId })
        .then((amount) => {
          this.updateState()
          alert("you harvested: " + amount + " caviar")
        })
    },
    harvest_stake() {
      window.contract
        .harvest_stake({ account_id: window.accountId })
        .then((amount) => {
          this.updateState()
          alert("you harvested: " + amount + " caviar")
        })
    },
    stakeCaviar() {
      window.contract
        .stake_caviar({ account_id: window.accountId, amount: parseInt(this.stakeInput.val) })
        .then((amount) => {
          this.updateState()
          alert("you staked: " + amount + " caviar")
        })
    },
    unstakeCaviar() {
      window.contract
        .unstake_caviar({ account_id: window.accountId })
        .then((amount) => {
          this.updateState()
          alert("you unstaked: " + amount + " caviar")
        })
    },
    stakePlus() {
      if (this.caviar > this.stakeInput.val) {
        this.stakeInput.val += 1
      }
    },
    stakeMinus() {
      if (this.stakeInput.val > 1) {
        this.stakeInput.val -= 1
      }
    },
    stakeMax() {
      this.stakeInput.val = this.caviar
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
