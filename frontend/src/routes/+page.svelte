<script lang="ts">
  import { GoogleOAuthProvider, googleLogout } from "google-oauth-gsi";
  import { onMount } from "svelte";

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
            console.log(token);
            time = null;
          },
        })();
      },
    });
  });

  type Time = {
    msg: string;
    time: string;
  };

  let time: Promise<Time> | null = null;
  let token: string | null = null;

  async function fetchTime(): Promise<Time> {
    const response = await fetch("/api/time", {
      headers: {
        Authorization: "Bearer " + token,
      },
    });
    if (!response.ok) {
      throw response;
    }
    return response.json();
  }
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
    <div>{time.msg}</div>
    <div>Current time: {time.time}</div>
  {:catch err}
    <div>Failed to fetch time: {err.status} {err.statusText}</div>
  {/await}
{/if}
