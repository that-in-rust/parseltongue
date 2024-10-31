/**
 * Configuration Pyramid:
 * L1: Core environment variables
 * L2: Backend configurations
 * L3: Theme settings
 * L4: Feature flags
 */

export const CONFIG = {
    // L1: Core environment
    ENV: process.env.NODE_ENV || 'development',
    API_TIMEOUT: 30000,
    POLL_INTERVAL: 1000,

    // L2: Backend configurations
    BACKENDS: {
        java: {
            url: process.env.JAVA_BACKEND_URL || 'http://localhost:8080',
            color: '#b07219',
            gradient: 'linear-gradient(90deg, #b07219, #d18f33)',
            maxRetries: 3
        },
        rust: {
            url: process.env.RUST_BACKEND_URL || 'http://localhost:8081',
            color: '#dea584',
            gradient: 'linear-gradient(90deg, #dea584, #e6b89c)',
            maxRetries: 3
        }
    },

    // L3: Theme settings
    THEME: {
        colors: {
            surface: '#0d1117',
            card: '#161b22',
            accent: '#238636',
            text: '#e6edf3',
            dim: '#7d8590'
        },
        fonts: {
            code: 'JetBrains Mono',
            ui: 'Inter'
        },
        spacing: {
            base: 4,
            card: 16,
            container: 24
        }
    },

    // L4: Feature flags
    FEATURES: {
        enableWebSockets: false,
        enableMetricsLogging: true,
        enablePerformanceTracking: true,
        debugMode: process.env.DEBUG === 'true'
    }
} as const;

export type BackendConfig = typeof CONFIG.BACKENDS[keyof typeof CONFIG.BACKENDS];
export type ThemeColors = typeof CONFIG.THEME.colors;