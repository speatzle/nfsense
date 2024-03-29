# nfSense Web Client
This folder contains the standard client for the nfSense firewall.

## Goals and Priorities
The nfSense web client should be...
- As reliable as possible
- Pleasing to the eye and intuitive to new users
- Fast, and not get in your way.

## Technology
### General Structure
The nfSense web client is a monolithic Single-Page Web-Application.

### Critical Dependencies
We encourage any potential contributor to familiarize themselves with these tools and libraries before making changes, as they lie at the heart of the project.
- Vue 3: Frontend Framework
- Vite: Dev and Build Tool
- Typescript: Adds Type Information to JS
- Vue Macros: Replacement for the deprecated experimental Reactivity Transform feature of Vue 3

### Important Dependencies
These libraries aren't mandatory to be able to work on the nfSense web client, this listing is merely meant to pre-empt redundancies.
- ESLint: The standard JS linter, which we also use to enforce some code style choices.
- Iconify: Icons (Could be replaced with custom ones)
- Markdown-It: We prefer markdown for informational texts, and use this to render it on the frontend.

### High-Level choices
We are currently not using a component library to maintain full control over our styling and keep the bundle small and dependencies few.

We do not use JavaScript-Based layouting and animations. Components simply render out DOM nodes that are styled and animated with CSS alone.

## Building
We use pnpm as our package manager please follow their documentation to set up a working development environment on the latest stable release of Node. Run `pnpm run dev` to start a development server, or `pnpm run build` to generate distribution files. We're working on including launch tasks to allow seamless debugging within VSCodium.
