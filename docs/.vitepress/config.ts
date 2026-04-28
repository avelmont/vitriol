import { withMermaid } from 'vitepress-plugin-mermaid'

// noinspection JSUnusedGlobalSymbols
export default withMermaid({
  title: "Vitriol",
  description: "Documentation for the Vitriol Project",
  mermaid: {
    theme: 'base',
    themeVariables: {
      primaryColor: '#8B5CF6',
      primaryTextColor: '#E2E8F0',
      primaryBorderColor: '#7C3AED',
      lineColor: '#94A3B8',
      secondaryColor: '#334155',
      tertiaryColor: '#1E293B',
      background: '#0F172A',
      mainBkg: '#1E293B',
      nodeBorder: '#334155',
      clusterBkg: '#1E293B',
      titleColor: '#E2E8F0',
      edgeLabelBackground: '#334155',
      attributeBackgroundColorEven: '#1E293B',
      attributeBackgroundColorOdd: '#0F172A',
      fontFamily: 'Lexend, sans-serif',
    }
  },
  mermaidPlugin: {
    class: 'mermaid'
  },
  themeConfig: {
    // https://vitepress.dev/reference/default-theme-config
    nav: [
      { text: 'Home', link: '/' },
    ],

    sidebar: [

    ],

    socialLinks: [
      { icon: 'github', link: 'https://github.com/avelmont/vitriol' }
    ]
  }
})
