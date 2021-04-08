/* eslint-disable @typescript-eslint/explicit-module-boundary-types */
import { reactive, toRefs, onBeforeMount } from "vue";
import { useRouter } from "vue-router";
import { Auth0Client, User } from "@auth0/auth0-spa-js";
import createAuth0Client from "@auth0/auth0-spa-js";

export interface AuthState {
  authenticating: boolean;
  authenticated: boolean;
  user: User | null;
}

const state = reactive<AuthState>({
  authenticating: true,
  authenticated: false,
  user: null,
});

let client: Auth0Client;

export function useAuthInit() {
  const router = useRouter();
  const auth = useAuth();
  function hasRedirected() {
    const urlParams = new URLSearchParams(window.location.search);
    return urlParams.has("code") && urlParams.has("state");
  }

  onBeforeMount(async () => {
    if (!client) {
      client = await createAuth0Client({
        domain: process.env.VUE_APP_AUTH0_DOMAIN,
        client_id: process.env.VUE_APP_AUTH0_CLIENT_ID,
        redirect_uri: process.env.VUE_APP_AUTH0_REDIRECT_URI,
        useRefreshTokens: true,
      });

      if (hasRedirected()) {
        await client.handleRedirectCallback();
        const user = await client.getUser();
        if (user) {
          auth.user.value = user;
        }
      }

      const authenticated = await client.isAuthenticated();

      auth.authenticated.value = authenticated;
      auth.authenticating.value = false;

      if (authenticated) {
        await router.push({ name: "ToDos" });
      } else {
        await router.push({ name: "Home" });
      }
    }
  });

  return {
    ...auth,
    router,
  };
}

export function useAuth() {
  async function login() {
    await client.loginWithRedirect();
  }

  async function logout() {
    await client.logout({
      returnTo: process.env.VUE_APP_AUTH0_RETURN_TO_URI,
    });
  }

  async function getAccessToken() {
    const accessToken = await client.getIdTokenClaims();
    return accessToken.__raw;
  }

  return {
    ...toRefs(state),
    login,
    logout,
    getAccessToken,
  };
}
