<script lang="ts">
  import { onMount } from "svelte";
  import { GoogleOAuthProvider, googleLogout } from "google-oauth-gsi";
  import { jwtDecode } from "jwt-decode";

  type Time = {
    time: string;
  };

  type User = {
    email: string;
    name: string;
    picture: string;
    exp: number;
  };

  let time: Promise<Time> | null = null;
  
  let token: string | null = null;
  let user: User | null = null;
  $: user = token === null ? null : (jwtDecode(token) as User);

  async function fetchTime(): Promise<Time> {
    const response = await fetch("/api/time", {
      method: "GET",
      headers: {
        "Content-Type": "application/json",
        Authorization: "Bearer " + token,
      },
    });
    if (!response.ok) {
      throw response;
    }
    return response.json();
  }

  function logout() {
    googleLogout();
    token = null;
    time = null;
  }

  onMount(() => {
    const googleProvider = new GoogleOAuthProvider({
      clientId:
        "981002175662-g8jr2n89bptsn8n9ds1fn5edfheojr7i.apps.googleusercontent.com",
      onScriptLoadSuccess: () => {
        googleProvider.useRenderButton({
          element: document.getElementById("login-button")!,
          use_fedcm_for_prompt: true,
          onSuccess: (res) => {
            token = res.credential || null;
            console.log(token)
            time = null;
          },
        })();
      },
    });
  });
</script>

<h1>Time Getter</h1>

<div style={token === null ? "" : "display:none"}>
  <div id="login-button"></div>
</div>

{#if token !== null}
  <button on:click={() => logout()}>Logout</button>
{/if}
<button on:click={() => (time = fetchTime())}>Get Time</button>
{#if time}
  {#await time}
    <div>Fetching time...</div>
  {:then time}
    <div>Current time: {time.time}</div>
  {:catch err}
    <div>Failed to fetch time: {err.status}, {err.statusText}</div>
  {/await}
{/if}
