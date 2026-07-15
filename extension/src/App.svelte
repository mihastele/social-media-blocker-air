<script>
  let site = '';
  let blockedSites = [];

  function addSite() {
    blockedSites = [...blockedSites, site];
    site = '';
    // Send message to background
    chrome.runtime.sendMessage({ type: 'UPDATE_BLOCK_LIST', blockedSites });
  }
</script>

<input bind:value={site} placeholder="Enter site" />
<button on:click={addSite}>Block</button>

<ul>
  {#each blockedSites as s}
    <li>{s}</li>
  {/each}
</ul>
