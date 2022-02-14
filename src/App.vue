<template>
  <div id="root">
    <nav class="navbar navbar-expand-lg navbar-dark bg-dark">
  <div class="container-fluid">
    <a class="navbar-brand" href="#">FISHA</a>
    <button class="navbar-toggler" type="button" data-bs-toggle="collapse" data-bs-target="#navbarColor03" aria-controls="navbarColor03" aria-expanded="false" aria-label="Toggle navigation">
      <span class="navbar-toggler-icon"></span>
    </button>

    <div class="collapse navbar-collapse" id="navbarColor03">
      <ul class="navbar-nav me-auto">
        <li class="nav-item">
          <a class="nav-link active" href="#">About
            <span class="visually-hidden">(current)</span>
          </a>
        </li>
        <!--li class="nav-item">
          <a class="nav-link" href="#">Features</a>
        </li>
        <li class="nav-item">
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
        <button class="btn btn-outline-primary" v-show="!isSignedIn" v-on:click="login">Sign In</button>
        <button class="btn btn-outline-info" v-show="isSignedIn" v-on:click="logout">Sign Out</button>
      </span>
    </div>
  </div>
</nav>
  <div class="container">
    <SignedOut v-show="!isSignedIn" />
    <SignedIn v-show="isSignedIn" />
  </div>
  </div>
</template>

<script>
import "./assets/bootstrap/css/bootstrap.min.css"
import "./global.css"
import "./assets/bootstrap/js/bootstrap.min.js"
import getConfig from "./config"
import { login, logout } from "./utils"
import SignedOut from "./components/SignedOut.vue"
import SignedIn from "./components/SignedIn.vue"

const nearConfig = getConfig("testnet")
//const nearConfig = getConfig(process.env.NODE_ENV || "development")
console.log(
  `networkId:${nearConfig.networkId} CONTRACT_NAME:${nearConfig.contractName}`
)
window.networkId = nearConfig.networkId

export default {
  created() {
    document.title = "fisha.co.in :: testnet"
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
  },
  methods: {
    login: login,
    logout: logout,
  },
}
</script>

