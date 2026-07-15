chrome.runtime.onMessage.addListener((message, sender, sendResponse) => {
  if (message.type === 'UPDATE_BLOCK_LIST') {
    const { blockedSites } = message;
    
    chrome.declarativeNetRequest.getDynamicRules((rules) => {
      const removeRuleIds = rules.map(rule => rule.id);
      
      const addRules = blockedSites.map((site, index) => ({
        id: index + 1,
        priority: 1,
        action: { type: 'block' },
        condition: {
          urlFilter: `*://${site}/*`,
          resourceTypes: ['main_frame']
        }
      }));

      chrome.declarativeNetRequest.updateDynamicRules({
        removeRuleIds,
        addRules
      }, () => {
        sendResponse({ status: 'success' });
      });
    });
    return true;
  }
});
