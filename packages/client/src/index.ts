import { AuthClient } from './auth.js'

const authClient = new AuthClient()

if (authClient.getStatus()) {
  document.querySelector('#status span')!.innerHTML = "logged in as "
} else {
  document.querySelector('#status span')!.innerHTML = "logged out"
}

async function updateStatusLabel() {
  if (!authClient.getStatus()) {
    return
  }
  const details = authClient.getDetails()
  if (!details) {
    document.querySelector('#status span')!.innerHTML = "logged out"
    return
  }
  document.querySelector('#status span')!.innerHTML = "logged in as " + details.email
}

updateStatusLabel()

authClient.onAuthAction(async (status, state) => {
  console.warn(status, state)
  updateStatusLabel()
})

document.querySelector('#btn_login')!.addEventListener('click', async () => {
  authClient.navigateToLogin({ hello: 'world' })
})

document.querySelector('#btn_logout')!.addEventListener('click', async () => {
  authClient.navigateToLogout()
})

document.querySelector('#btn_refresh')!.addEventListener('click', async () => {
  await authClient.refreshAuth()
})

document.querySelector('#btn_protected')!.addEventListener('click', async () => {
  const resp = await authClient.validate()
  console.log(resp)
})
