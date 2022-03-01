<template>
  <div id="root">
    <nav class="navbar navbar-expand-lg navbar-dark ">
  <div class="container-fluid">
    <a class="navbar-brand" href="#">FISHA</a>
    <button class="navbar-toggler" type="button" data-bs-toggle="collapse" data-bs-target="#navbarColor03" aria-controls="navbarColor03" aria-expanded="false" aria-label="Toggle navigation">
      <span class="navbar-toggler-icon"></span>
    </button>

    <div class="collapse navbar-collapse" id="navbarColor03">
      <ul class="navbar-nav me-auto">
        <li class="nav-item" v-show="!isSignedIn">
          <a class="nav-link active" href="#" data-bs-toggle="modal" data-bs-target="#roadmap">
            <font-awesome-icon icon="fa-solid fa-road" />
            Roadmap
            <span class="visually-hidden">(current)</span>
          </a>
        </li>
        <li class="nav-item" v-show="!isHasLocker && isSignedIn">
          <a class="nav-link" href v-on:click.prevent="initLocker">
            <font-awesome-icon icon="fa-solid fa-circle-play" />
            HODL & Start</a>
        </li>
        <li class="nav-item " v-show="isHasLocker">
          <a class="nav-link disabled" href >
            <font-awesome-icon icon="fa-solid fa-money-bill-1" />
            Withdraw (365 days to unlock)
          </a>
        </li>
        <li class="nav-item" v-show="isHasLocker">
          <a class="nav-link" href v-on:click.prevent="retrieveRandom">
            <font-awesome-icon icon="fa-solid fa-face-grin-stars" />
            Lottery
          </a>
        </li>
        <!--li class="nav-item">
          <a class="nav-link" href="#">Pricing</a>
        </li>
        <li class="nav-item">
          <a class="nav-link" href="#">About</a>
        </li>
        <li class="nav-item dropdown">
          <a class="nav-link dropdown-toggle" data-bs-toggle="dropdown" href="#" role="button" aria-haspopup="true" aria-expanded="false">Dropdown</a>
          <div class="dropdown-menu">
            <a class="dropdown-item" href="#">Action</a>
            <a class="dropdown-item" href="#">Another action</a>
            <a class="dropdown-item" href="#">Something else here</a>
            <div class="dropdown-divider"></div>
            <a class="dropdown-item" href="#">Separated link</a>
          </div>
        </li -->
      </ul>
      <span class="d-flex">
        <a class="btn btn-primary btn-sm" style="margin-right:1rem;" href="https://github.com/ioustamora/fisha" v-show="!isSignedIn">
          <font-awesome-icon icon="fa-brands fa-github-alt" />
        </a>
        <button class="btn btn-secondary btn-sm disabled" style="margin-right:1rem;" v-show="isSignedIn">
          <font-awesome-icon icon="fa-solid fa-user-secret" />
          {{ accountId }} {{ accountBalance.available.trimEnd(16) }} NEAR
        </button>
        <button class="btn btn-outline-light btn-sm" v-show="!isSignedIn" v-on:click="login">Sign In</button>
        <button class="btn btn-outline-light btn-sm" v-show="isSignedIn" v-on:click="logout">Sign Out</button>
      </span>
    </div>
  </div>
</nav>
  <div class="container">
    <SignedOut v-show="!isSignedIn" />
    <SignedIn ref="foo" v-show="isSignedIn" />
  </div>
  <!-- Modal -->
<div class="modal fade" id="roadmap" tabindex="-1" aria-labelledby="exampleModalLabel" aria-hidden="true">
  <div class="modal-dialog">
    <div class="modal-content">
      <div class="modal-header">
        <h5 class="modal-title" id="exampleModalLabel">Roadmap</h5>
        <button type="button" class="btn-close" data-bs-dismiss="modal" aria-label="Close"></button>
      </div>
      <div class="modal-body">
        <div class="accordion" id="accordionExample">
  <div class="accordion-item">
    <h2 class="accordion-header" id="headingOne">
      <button class="accordion-button btn-success" type="button" data-bs-toggle="collapse" data-bs-target="#collapseOne" aria-expanded="true" aria-controls="collapseOne">
        Step #1 (current) Idea to MVP.
      </button>
    </h2>
    <div id="collapseOne" class="accordion-collapse collapse show" aria-labelledby="headingOne" data-bs-parent="#accordionExample">
      <div class="accordion-body bg-success">
        <form>
          <fieldset>
            <fieldset class="form-group">
              <div class="form-check">
                <input class="form-check-input" type="checkbox" value="" id="flexCheckDefault" checked="">
                <label class="form-check-label" for="flexCheckDefault">
                  hodl & start
                </label>
              </div>
              <div class="form-check">
                <input class="form-check-input" type="checkbox" value="" id="flexCheckChecked" checked="">
                <label class="form-check-label" for="flexCheckChecked">
                  lottery
                </label>
              </div>
              <div class="form-check">
                <input class="form-check-input" type="checkbox" value="" id="flexCheck1" checked="">
                <label class="form-check-label" for="flexCheck1">
                  harvest caviar from assets
                </label>
              </div>
              <div class="form-check">
                <input class="form-check-input" type="checkbox" value="" id="flexCheck2" checked="">
                <label class="form-check-label" for="flexCheck2">
                  staking caviar
                </label>
              </div>
              <div class="form-check">
                <input class="form-check-input" type="checkbox" value="" id="flexCheck3" checked="">
                <label class="form-check-label" for="flexCheck3">
                  swap/exchange assets
                </label>
              </div>
              <div class="form-check">
                <input class="form-check-input" type="checkbox" value="" id="flex1" checked="">
                <label class="form-check-label" for="flex1">
                  FISHA token
                </label>
              </div>
              <div class="form-check">
                <input class="form-check-input" type="checkbox" value="" id="flex10">
                <label class="form-check-label" for="flex1">
                  test FISHA token on testnet
                </label>
              </div>
              <div class="form-check">
                <input class="form-check-input" type="checkbox" value="" id="flex11">
                <label class="form-check-label" for="flex1">
                  deploy FISHA token on testnet
                </label>
              </div>
              <div class="form-check">
                <input class="form-check-input" type="checkbox" value="" id="flex2">
                <label class="form-check-label" for="flex2">
                  exchange caviar to FISHA
                </label>
              </div>
              <div class="form-check">
                <input class="form-check-input" type="checkbox" value="" id="flex1">
                <label class="form-check-label" for="flex1">
                  FISHA token staking
                </label>
              </div>
            </fieldset>
          </fieldset>
        </form>
      </div>
    </div>
  </div>
  <div class="accordion-item">
    <h2 class="accordion-header" id="headingTwo">
      <button class="accordion-button btn-info collapsed" type="button" data-bs-toggle="collapse" data-bs-target="#collapseTwo" aria-expanded="false" aria-controls="collapseTwo">
        Step #2
      </button>
    </h2>
    <div id="collapseTwo" class="accordion-collapse collapse" aria-labelledby="headingTwo" data-bs-parent="#accordionExample">
      <div class="accordion-body bg-info">
        Test app and in-game economy model.
      </div>
    </div>
  </div>
  <div class="accordion-item">
    <h2 class="accordion-header" id="headingThree">
      <button class="accordion-button btn-info collapsed" type="button" data-bs-toggle="collapse" data-bs-target="#collapseThree" aria-expanded="false" aria-controls="collapseThree">
        Step #3
      </button>
    </h2>
    <div id="collapseThree" class="accordion-collapse collapse" aria-labelledby="headingThree" data-bs-parent="#accordionExample">
      <div class="accordion-body bg-info">
        Deploy FISHA to mainnet. Start)
      </div>
    </div>
  </div>
</div>
      </div>
    </div>
  </div>
</div>
  </div>
</template>

<script>
import "./assets/bootstrap/css/bootstrap.min.css"
import "./global.css"
import "./assets/bootstrap/js/bootstrap.min.js"
import getConfig from "./config"
import { login, logout } from "./utils"
import { utils } from 'near-api-js'
import SignedOut from "./components/SignedOut.vue"
import SignedIn from "./components/SignedIn.vue"

const nearConfig = getConfig("testnet")
//const nearConfig = getConfig(process.env.NODE_ENV || "development")
console.log(
  `networkId:${nearConfig.networkId} CONTRACT_NAME:${nearConfig.contractName}`
)
window.networkId = nearConfig.networkId

export default {
  data: function () {
    return {
      isHasLocker: false,
      accountBalance: {
          "total": "0",
          "stateStaked": "0",
          "staked": "0",
          "available": "0"
      },
    }
  },
  created() {
    document.title = "fisha.co.in :: testnet"
    if (this.isSignedIn) {
      this.hasLocker()
      this.getAccountBalance()
    }
  },
  name: "App",
  components: {
    SignedOut,
    SignedIn,
  },

  computed: {
    isSignedIn() {
      return window.walletConnection.isSignedIn()
    },
    accountId() {
      return window.accountId
    },
  },
  methods: {
    login: login,
    logout: logout,
    initLocker() {
      window.contract
        .init_locker({}, "300000000000000", "1000000000000000000000000")
        .then(() => {
          console.log("start init_locker");
        })
    },
    hasLocker() {
      window.contract
        .has_locker({ account_id: window.accountId })
        .then((result) => {
          this.isHasLocker = result
          console.log("has locker = ", result);
        })
    },
    getAccountBalance () {
      let self = this;
        window.walletConnection.account().getAccountBalance()
      .then(function(result){
        console.log(result)
        self.accountBalance = result;
        self.accountBalance.total = utils.format.formatNearAmount(self.accountBalance.total);
        self.accountBalance.stateStaked = utils.format.formatNearAmount(self.accountBalance.stateStaked);
        self.accountBalance.staked = utils.format.formatNearAmount(self.accountBalance.staked);
        self.accountBalance.available = utils.format.formatNearAmount(self.accountBalance.available, 8);
      })
    },
    retrieveRandom() {
      //retrieve random 
      window.contract
        .get_random({ account_id: window.accountId })
        .then((random) => {
          this.$children[1].updateState()
          alert(random)
        })
    },
  },
}
</script>

