<script lang="ts">
  // searching function and query
  let { on_search, initial_search } = $props();
  let query = $state(initial_search);
  
  // debouncing
  const DEBOUNCE_DURATION_MS = 300;
  let timeout_debounce: ReturnType<typeof setTimeout> | null = null;

  // emit on_search every time query changes, with some debounce
  $effect(() => {
    // need this to track changes in query (svelte 5)
    if (query == "") { return; }

    if (timeout_debounce) {
      clearTimeout(timeout_debounce);
    }

    timeout_debounce = setTimeout(() => {
      if (query.trim() !== '') {
        on_search?.({ search: query });
      }
    }, DEBOUNCE_DURATION_MS);
  });
</script>

<div class="sticky top-0 flex justify-center m-2">
  <input type="text" class="border-2 border-brown-400 p-2 flex-grow" bind:value={query} placeholder="Search..." />
</div>

