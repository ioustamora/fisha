import * as Vue from "vue/dist/vue"
import App from "./App.vue"

/* import the fontawesome core */
import { library } from '@fortawesome/fontawesome-svg-core'

/* import specific icons */
import { 
  faUserSecret, 
  faCirclePlay, 
  faFaceGrinStars, 
  faRoad, 
  faMoneyBill1,
  faVault,
  faFish, 
} from '@fortawesome/free-solid-svg-icons'
import { faGithubAlt } from '@fortawesome/free-brands-svg-icons'

/* import font awesome icon component */
import { FontAwesomeIcon } from '@fortawesome/vue-fontawesome'

/* add icons to the library */
library.add(
  faUserSecret, 
  faGithubAlt, 
  faCirclePlay, 
  faFaceGrinStars, 
  faMoneyBill1, 
  faRoad,
  faFish,
  faVault,
)

/* add font awesome icon component */
Vue.component('font-awesome-icon', FontAwesomeIcon)

import { initContract } from "./utils"

Vue.config.productionTip = false

window.nearInitPromise = initContract()
  .then(() => {

    new Vue({
      render: h => h(App),
    }).$mount("#app")

  })
  