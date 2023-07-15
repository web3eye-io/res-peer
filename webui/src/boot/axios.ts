import { boot } from 'quasar/wrappers'
import axios, { AxiosInstance, AxiosResponse, AxiosError } from 'axios'

declare module '@vue/runtime-core' {
  interface ComponentCustomProperties {
    $axios: AxiosInstance;
  }
}

// Be careful when using SSR for cross-request state pollution
// due to creating a Singleton instance here;
// If any client changes this (global) instance, it might be a
// good idea to move this instance creation inside of the
// "export default () => {}" function below (which runs individually
// for each client)
const api = axios.create()

// define common response handle
interface CommonError {
  code: number
  message: string
}

const post = async <T, R>(url: string, data: T) => {
  return await api
    .post<T, AxiosResponse<R>>(url, data)
    .then((data: AxiosResponse<R>) => data.data)
    .catch((err: AxiosError<CommonError>) => {
      throw new Error(err.response?.data.message || err.message) // err.message is uncached error
    })
}

export default boot(({ app }) => {
  // for use inside Vue files (Options API) through this.$axios and this.$api

  app.config.globalProperties.$axios = axios
  // ^ ^ ^ this will allow you to use this.$axios (for Vue Options API form)
  //       so you won't necessarily have to import axios in each vue file

  app.config.globalProperties.$api = api
  // ^ ^ ^ this will allow you to use this.$api (for Vue Options API form)
  //       so you can easily perform requests against your app's API
})

// export api, post method recommand post
export { api, post }
