<script lang="ts">
  import SearchBar from "./lib/SearchBar.svelte";
  import ResultCard from "./lib/ResultCard.svelte";
  import Landing from "./lib/Landing.svelte";
  import Header from "./lib/Header.svelte";

  let search_results: any[] = $state([]);
  let search_started = $state(false);

  // initial search
  const SEARCH_TERMS = [
    "id ego superego",
    "course on making sick beats",
    "building a motherboard from scratch",
    "how to become tiktok famous",
    "uzbek cuisine and culture",
    "i want to learn how to dance",
    "i love planet earth",
    "where can i learn about the stock market",
    "how to become a great chef",
    "how do i protest for a good cause",
    "a course on designing a great city",
    "launching my own company",
  ];
  let random_search_term =
    SEARCH_TERMS[Math.floor(Math.random() * SEARCH_TERMS.length)];

  // initial search with random search term
  search({ search: random_search_term }, true);

  // Handle search
  function search(query: { search: string }, is_initial: boolean = false) {
    // fetch search results from localhost:3000
    fetch("/search", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify(query),
    })
      .then((res) => res.json())
      .then((data) => {
        search_results = data;
        search_started = true && !is_initial;
      })
      .catch((err) => {
        console.error(err);
      });
  }
</script>

<div class="mx-auto w-full max-w-screen-sm md:max-w-screen-md">
  <Header />
  <Landing />
  <SearchBar on_search={search} initial_search={random_search_term} />
  {#if search_results.length > 0}
    <ul>
      {#each search_results as result}
        <ResultCard {result} />
      {/each}
    </ul>
  {/if}
</div>
