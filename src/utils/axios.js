import axios from 'axios'
import { createToast } from 'mosha-vue-toastify'

 
// create an axios instance
const service = axios.create({
  baseURL: BaseUrl, // url = base url + request url
  //timeout: 5000 // request timeout
  timeout: 500000, // request timeout
})

// request interceptor
service.interceptors.request.use(
  (config) => {
    return config
  },
  (error) => {
    console.log(error) // for debug
    return Promise.reject(error)
  }
)

// response interceptor
service.interceptors.response.use(
  (response) => {
    const res = response.data
    if (res.code !== 20000) {
      console.error("http return ", res)
      createToast(
        { title: '', description: res.desc },
        {
          type: 'danger',
          showIcon: true,
          position: 'top-center',
          timeout: 8000,
        }
      )
      return Promise.reject(new Error(res.message || 'Error'))
    } else {
      return res.data
    }
  },
  (error) => {
    console.error('response err' + error) // for debug
    return Promise.reject(error)
  }
)

export default service
