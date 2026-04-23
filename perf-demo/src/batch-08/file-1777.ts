import type { Type1776 } from '../batch-08/file-1776.js';
export interface Type1777 {
  id: 1777;
  name: 'File1777';
  next: Type1776;
}

export function make1777(): Type1777 {
  return { id: 1777, name: 'File1777', next: null as unknown as Type1776 };
}
