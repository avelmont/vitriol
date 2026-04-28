---
# https://vitepress.dev/reference/default-theme-home-page
layout: home

hero:
  name: "Vitriol"
  text: Reactive workflow engine based on files
  tagline: Your data gets the information it needs for execution.
  actions:
    - theme: brand
      text: Source Code
      link: https://github.com/avelmont/vitriol
    - theme: alt
      text: Guide
      link: /guide
    - theme:
      text: Architectural Overview
      link: /architectural-overview
features:
  - title: Reactive
    details: >
      Your tasks don't need to wait for everything to be ready.
      the process start as soon as some data is available.
  - title: Declarative
    details: >
      Workflows are defined in KDL files, using a simple and structured syntax.
      It allows you to define your processes, inputs and outputs, and connect them with reactive channels.
  - title: Repeatability
    details: >
      Your workflows can be run multiple times. each workflow is fingerprinted, ensuring that each output is
      stable and reproducible.
---

