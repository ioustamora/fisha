import * as Vue from "vue/dist/vue"
import App from "./App.vue"

import { initContract } from "./utils"

Vue.config.productionTip = false

window.nearInitPromise = initContract()
  .then(() => {

    new Vue({
      render: h => h(App),
    }).$mount("#app")

  })
  