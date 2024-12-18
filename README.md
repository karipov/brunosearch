# Brunosearch

[**brunosearch.com**](http://brunosearch.com) is a course search engine for Brown University. Users can insert a rough description of what they're looking for in a course, which is then transformed into a semantic embedding powered by OpenAI. Search terms don't have to be precise, examples include "how to make sick beats" or "a class on the intersection of art and technology."

## Motivation

As registration looms near or shopping period is in full swing, I've often found myself frantically searching for a fun course to offset a painful looking shopping cart. Groupchats get flooded with messages like "does anyone know a fun class to take?" or "what's a class that's not too hard but also interesting?"

Brown already has a number of tools to help out here, like [The Critical Review](https://thecriticalreview.org/), [BurntOutAtBrown](https://burntoutatbrown.com/) and of course, [CAB](https://cab.brown.edu). While these are great resources, it's not always easy to find classes that are fun and aligned with my interests. I wanted to build something that would allow searching for courses in a more natural way, without having to know exactly what I'm looking for -- just like how I'd ask a friend for recommendations.

## Technical Details

**Brunosearch** is a custom, distributed search engine written in [Rust](https://www.rust-lang.org/), using an in-memory [Redis](https://redis.io/) vector database for similarity search. The embedding is powered by OpenAI's [text-embedding-3-small](https://openai.com/index/new-embedding-models-and-api-updates/) model, which scores well on most benchmarks and simplifies deployment in the cloud. The frontend is built in [Svelte](https://svelte.dev/) and [TailwindCSS](https://tailwindcss.com/). The entire app is hosted on [Fly.io](https://fly.io/) for easy deployment and scaling.

<!-- ## Development

You need Rust, Node.js  -->


## Acknowledgements

Created by Komron Aripov. I took a lot of inspiration from Eric Zhang's [classes.wtf](https://classes.wtf/) and [Dispict](https://dispict.com/) projects. Much of the CAB scraping code came directly from BurntOutAtBrown's [course scraper](https://github.com/KevinCox9600/burnt-out-at-brown), which was a huge help.