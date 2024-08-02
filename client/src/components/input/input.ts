import { Index } from '~/util';
// --- DROPDOWN_INPUT ---
export type Options = Record<Index, Option>;
export type Option = {
  [key: Index]: any, // Allow additional properties for customization
  display?: string,
};
export type SearchProvider = (opts: SearchOptions) => Promise<Options>;
export type MaybeSearchProvider = SearchProvider | null;
export type SearchOptions = {
  search: string,
  unknownKeys?: Index[],
  // parentData?: any,
};

// --- FORM INPUT ---
export type Field = {
  is: Component | string,
  label?: string,
  props?: any,
  // actions?: Action[],
  // forceCastNumber`?: bool,
};
export type Fields = Record<Index, Field>;

// --- ENUM INPUT ---
export type Variant = {
  fields?: Fields,
  display?: string,
  icon?: Component
};
export type Variants = Record<Index, Variant>;
export type EnumValueWithFields = { [index: Index]: Record<Index, any> }
export type EnumValue = Index | EnumValueWithFields;
export type MaybeEnumValue = EnumValue | null;
