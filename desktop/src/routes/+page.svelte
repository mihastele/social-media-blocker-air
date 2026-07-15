<script>
  let newSite = "";
  let blockedSites = ["example.com", "facebook.com"];

  function addSite() {
    if (newSite) {
      blockedSites = [...blockedSites, newSite];
      newSite = "";
    }
  }

  function removeSite(site) {
    blockedSites = blockedSites.filter(s => s !== site);
  }
</script>

<h1>Focus Blocker</h1>
<input bind:value={newSite} placeholder="Add site to block" />
<button on:click={addSite}>Add</button>

<ul>
  {#each blockedSites as site}
    <li>
      {site}
      <button on:click={() => removeSite(site)}>Remove</button>
    </li>
  {/each}
</ul>
