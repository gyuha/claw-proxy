const ADMIN_BASE = 'http://localhost:47381';

export async function getStatus() {
  const res = await fetch(`${ADMIN_BASE}/status`);
  return res.json();
}

export async function getProviders() {
  const res = await fetch(`${ADMIN_BASE}/providers`);
  return res.json();
}
