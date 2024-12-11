
export type ActionType = 'login' | 'logout' | 'refresh' | 'validate'

export class AuthClient {
  #listeners: Set<(action: ActionType, state: any | undefined) => any>

  #buffer: Array<[ActionType, any | undefined]>

  #status: boolean

  #details: any

  constructor() {
    this.#listeners = new Set()
    this.#buffer = []

    // await window.cookieStore.get('auth_refresh_expires')
    const cookies = parseCookie(document.cookie)
    const authAction = window.sessionStorage.getItem('auth_action')
    const previousState = window.sessionStorage.getItem('auth_state')
    const hasRefreshToken = cookies['auth_refresh_valid'] || false

    this.#status = hasRefreshToken

    if (cookies['auth_payload']) {
      this.#details = cookies['auth_payload']
    }

    if (authAction && previousState) {
      // @ts-expect-error
      this.#buffer.push([authAction, JSON.parse(previousState)])
    } else if (authAction && !previousState) {
      // @ts-expect-error
      this.#buffer.push([authAction, undefined])
    } else if (hasRefreshToken) {
      this.refreshAuth()
    }

    window.sessionStorage.removeItem('auth_action')
    window.sessionStorage.removeItem('auth_state')
  }

  getStatus() {
    return this.#status
  }

  getDetails() {
    return this.#details
  }

  onAuthAction(
    callback: (action: ActionType, state: any | undefined) => any
  ): () => any {
    this.#listeners.add(callback)
    while (this.#buffer.length) {
      // @ts-expect-error
      callback(...this.#buffer.shift())
    }
    return () => this.#listeners.delete(callback)
  }

  navigateToLogin(
    state: any | undefined
  ) {
    const target = new URL(window.location.origin)
    target.pathname = '/api/auth/login'

    window.sessionStorage.setItem('auth_action', 'login')
    if (state) {
      window.sessionStorage.setItem('auth_state', JSON.stringify(state))
    }

    window.location.assign(target)
  }

  navigateToLogout(
    state?: Record<string, any> | undefined
  ) {
    const target = new URL(window.location.origin)
    target.pathname = '/api/auth/logout'

    window.sessionStorage.setItem('auth_action', 'logout')
    if (state) {
      // target.searchParams.set('state', encodeURIComponent(btoa(JSON.stringify({ return_url: window.location.href }))))
      window.sessionStorage.setItem('auth_state', JSON.stringify(state))
    }

    window.location.assign(target)
  }

  async refreshAuth() {
    const target = new URL(window.location.origin)
    target.pathname = '/api/auth/refresh'
    const response = await fetch(target)
    if (!response.ok) {
      throw new Error('Invalid auth')
    }
    this.#details = await response.json()
    this.#notifyListeners('refresh', undefined)
  }

  async validate() {
    const target = new URL(window.location.origin)
    target.pathname = '/api/auth/validate'
    const response = await fetch(target)
    if (!response.ok) {
      throw new Error('Invalid auth')
    }
    this.#details = await response.json()
    this.#notifyListeners('validate', undefined)
    return this.#details
  }

  #notifyListeners(
    action: ActionType,
    state: any,
  ) {
    this.#listeners.forEach(listener => listener(action, state))
  }
}

export const parseCookie = (str= ''): Record<string, any>  =>
  str.split(';')
  .reduce((res, c) => {
    const [key, val] = c.trim().split('=').map(decodeURIComponent)
    try {
      return Object.assign(res, { [key]: JSON.parse(val) })
    } catch (e) {
      return Object.assign(res, { [key]: val })
    }
  }, {});
