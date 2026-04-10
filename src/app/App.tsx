function App() {
  const sectionCard = {
    padding: '20px',
    border: '1px solid rgba(0, 0, 0, 0.1)',
    borderRadius: '16px',
    background: '#ffffff',
    boxShadow: 'rgba(0, 0, 0, 0.02) 0px 8px 24px',
  } as const;

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
          display: 'grid',
          gap: '20px',
        }}
      >
        <header
          style={{
            ...sectionCard,
            padding: '32px',
            display: 'grid',
            gap: '18px',
          }}
        >
          <div>
            <p
              style={{
                margin: 0,
                fontSize: '12px',
                letterSpacing: '0.12em',
                textTransform: 'uppercase',
              }}
            >
              Local control plane
            </p>
            <h1 style={{ margin: '12px 0 8px', fontSize: '40px', lineHeight: 1.05 }}>
              Claw Proxy
            </h1>
            <p style={{ margin: 0, maxWidth: '48rem', color: '#615d59', fontSize: '16px' }}>
              A calm desktop shell for managing the embedded runtime, future provider
              connections, and routing policy from one place.
            </p>
          </div>

          <div
            aria-label="Runtime status"
            style={{
              padding: '18px 20px',
              borderRadius: '14px',
              background: '#f6f5f4',
              border: '1px solid rgba(0, 0, 0, 0.08)',
              display: 'flex',
              justifyContent: 'space-between',
              alignItems: 'center',
              gap: '16px',
            }}
          >
            <div>
              <p style={{ margin: 0, fontSize: '12px', textTransform: 'uppercase', letterSpacing: '0.1em' }}>
                Runtime Status
              </p>
              <strong style={{ display: 'block', marginTop: '6px', fontSize: '20px' }}>
                Host bootstrap ready
              </strong>
              <p style={{ margin: '6px 0 0', color: '#615d59' }}>
                The Rust host is connected and ready for typed runtime state in the next
                execution wave.
              </p>
            </div>
            <span
              style={{
                padding: '6px 12px',
                borderRadius: '999px',
                background: '#f2f9ff',
                color: '#097fe8',
                fontSize: '12px',
                fontWeight: 700,
                letterSpacing: '0.08em',
                textTransform: 'uppercase',
              }}
            >
              Ready
            </span>
          </div>
        </header>

        <section
          aria-label="Future product areas"
          style={{
            display: 'grid',
            gridTemplateColumns: 'repeat(auto-fit, minmax(200px, 1fr))',
            gap: '16px',
          }}
        >
          {[
            ['Providers', 'Securely connect multiple AI providers and account pools.'],
            ['Routing', 'Control default behavior, failover, and request priorities.'],
            ['Diagnostics', 'Surface health, logs, and recovery hints from the runtime.'],
            ['Settings', 'Manage local proxy ports, startup behavior, and host preferences.'],
          ].map(([title, description]) => (
            <article key={title} style={sectionCard}>
              <p
                style={{
                  margin: 0,
                  fontSize: '12px',
                  textTransform: 'uppercase',
                  letterSpacing: '0.08em',
                  color: '#a39e98',
                }}
              >
                Upcoming area
              </p>
              <h2 style={{ margin: '12px 0 8px', fontSize: '22px' }}>{title}</h2>
              <p style={{ margin: 0, color: '#615d59', lineHeight: 1.5 }}>{description}</p>
            </article>
          ))}
        </section>
      </section>
    </main>
  );
}

export default App;
