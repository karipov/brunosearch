<script lang="ts">
    interface Course {
        department_full: string;
        department_short: string;
        code: string;
        title: string;
        professor: string;
        time: string;
        description: string;
        writ: boolean;
        soph: boolean;
        fys: boolean;
        rpp: boolean;
    }
    let { result }: {result: Course} = $props();

    let expanded_description = $state(false);
</script>

<div class="border-2 shadow-sm p-2 bg-white m-2 transition-shadow duration-300 ease-in-out hover:shadow-lg">
    <div class="flex justify-between items-start mb-2">
        <!-- Title Section -->
        <p class="flex-grow max-w-full mr-1 font-semibold">
            <strong class="font-extrabold">{result.department_short} {result.code}:</strong> {result.title}
        </p>
        <!-- Tags Section -->
        <div class="flex flex-shrink-0 space-x-1 text-xs font-semibold">
            {#if result.writ}
            <span class="bg-red-200 text-red-800 px-2.5 py-0.5">WRIT</span>
            {/if}
            {#if result.soph}
            <span class="bg-green-200 text-green-800 px-2.5 py-0.5">SOPH</span>
            {/if}
            {#if result.fys}
            <span class="bg-yellow-200 text-yellow-800 px-2.5 py-0.5">FYS</span>
            {/if}
            {#if result.rpp}
            <span class="bg-brown-100 text-brown-800 px-2.5 py-0.5">RPP</span>
            {/if}
        </div>
    </div>
    <p class="text-gray-700 text-sm mb-2">
        {result.professor} | {result.time}
    </p>
    <div class="text-gray-700 text-sm mb-2">
        <!-- svelte-ignore
         a11y_no_noninteractive_element_interactions,
         a11y_click_events_have_key_events -->
        <p
          class={`cursor-pointer ${
            expanded_description ? 'line-clamp-none' : 'line-clamp-2'
          }`}
          onclick={() => expanded_description = !expanded_description} >
            {result.description}
        </p>
        {#if expanded_description}
          <button
            class="text-blue-500 hover:text-blue-700 transition-colors duration-200 mt-1"
            onclick={() => expanded_description = false}
          >
            Read Less
          </button>
        {/if}
      </div>

</div>


