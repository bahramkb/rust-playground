# Rust + React + NX monorepo

This repository is for playing around with the setup and idea of having a monorepo to use with Rust and React using NX as the monorepo manager.

This repository uses `pnpm` as its package manager.

## Getting started

1. Clone the repository

2. Run `pnpm install`

## Add new projects

### React

Use the plugin's generator to create new projects.
To create a new React app or library:

```sh
# Genenerate an app
npx nx g @nx/react:app demo

# Generate a library
npx nx g @nx/react:lib some-lib
```

### Rust

To create a new Rust app or library:

```sh
# Genenerate an app
nx g @monodon/rust:binary myapp

# Generate a library
nx g @monodon/rust:library cats
```

## Run an existing project

### React

To serve a react application use:
```sh
pnpm nx run react-monorepo:serve
```

To test it, run: 
```sh
pnpm nx run react-monorepo:test
```

### Rust

To build a library and to test it, run:
```sh
pnpm nx run cats:build
pnpm nx run cats:test
```

To build an application and to test it, run:
```sh
pnpm nx run myapp:test
pnpm nx run myapp:run
```