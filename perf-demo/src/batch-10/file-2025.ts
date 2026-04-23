import type { Type2024 } from '../batch-10/file-2024.js';
export interface Type2025 {
  id: 2025;
  name: 'File2025';
  next: Type2024;
}

export function make2025(): Type2025 {
  return { id: 2025, name: 'File2025', next: null as unknown as Type2024 };
}
