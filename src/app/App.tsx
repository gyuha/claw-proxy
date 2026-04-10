function App() {
  return (
    <main
      style={{
        minHeight: '100vh',
        padding: '48px',
        fontFamily:
          '"NotionInter", Inter, -apple-system, BlinkMacSystemFont, "Segoe UI", sans-serif',
        background:
          'linear-gradient(180deg, rgba(246, 245, 244, 0.92) 0%, #ffffff 55%)',
        color: 'rgba(0, 0, 0, 0.95)',
      }}
    >
      <section
        aria-label="Application shell"
        style={{
          maxWidth: '960px',
          margin: '0 auto',
          padding: '32px',
          border: '1px solid rgba(0, 0, 0, 0.1)',
          borderRadius: '20px',
          background: '#ffffff',
          boxShadow:
            'rgba(0, 0, 0, 0.04) 0px 4px 18px, rgba(0, 0, 0, 0.02) 0px 1px 4px',
        }}
      >
        <p style={{ margin: 0, fontSize: '12px', letterSpacing: '0.12em', textTransform: 'uppercase' }}>
          Local control plane
        </p>
        <h1 style={{ margin: '12px 0 8px', fontSize: '40px', lineHeight: 1.05 }}>
          Claw Proxy
        </h1>
        <p style={{ margin: 0, maxWidth: '48rem', color: '#615d59', fontSize: '16px' }}>
          React and Tauri are wired together, and the Rust host is ready to become the
          authority for runtime state.
        </p>
      </section>
    </main>
  );
}

export default App;
