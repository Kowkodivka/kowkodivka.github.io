import * as i18n from "@solid-primitives/i18n";
import type * as en from "@/locales/en-US.ts";
import {
  type Accessor,
  createContext,
  createResource,
  createSignal,
  type ParentComponent,
  Resource,
  useContext,
  useTransition,
} from "solid-js";

interface Locale {
  locale: string;
  label: string;
  name: string;
}

type RawDictionary = typeof en.dict;
type Dictionary = i18n.Flatten<RawDictionary>;

const locales: Locale[] = [
  { locale: "en-US", label: "EN", name: "English" },
  { locale: "ru", label: "RU", name: "Русский" },
];

interface I18nContextValue {
  t: i18n.NullableTranslator<Dictionary>;
  dict: Resource<Dictionary>;
  currentLocale: Accessor<Locale>;
  setCurrentLocale: (locale: Locale) => void;
  isTransitioning: Accessor<boolean>;
}

export const I18nContext = createContext<I18nContextValue>();

const I18nProvider: ParentComponent = (props) => {
  function getInitialLocale(): Locale {
    const savedLocale = localStorage.getItem("locale");

    if (savedLocale) {
      const match = locales.find((l) => l.locale === savedLocale);
      if (match) return match;
    }

    for (const language of navigator.languages) {
      const match = locales.find((l) => l.locale === language);
      if (match) return match;
    }

    return locales[0];
  }

  async function fetchDictionary(locale: Locale): Promise<Dictionary> {
    const { dict }: { dict: RawDictionary } = await import(
      `@/locales/${locale.locale}.ts`
    );
    return i18n.flatten(dict);
  }

  const [locale, setLocale] = createSignal<Locale>(getInitialLocale());
  const [isTransitioning, startTransition] = useTransition();

  const setCurrentLocale = (newLocale: Locale) => {
    if (newLocale.locale === locale().locale) return;

    startTransition(() => {
      setLocale(newLocale);
    });

    localStorage.setItem("locale", newLocale.locale);
  };

  const [dict] = createResource(locale, fetchDictionary);
  const t = i18n.translator(dict);

  const value: I18nContextValue = {
    t,
    dict,
    currentLocale: locale,
    setCurrentLocale,
    isTransitioning,
  };

  return (
    <I18nContext.Provider value={value}>{props.children}</I18nContext.Provider>
  );
};

function useI18n() {
  const ctx = useContext(I18nContext);
  if (!ctx) throw new Error("useI18n must be used inside I18nProvider");
  return ctx;
}

export {
  type Locale,
  type RawDictionary,
  type Dictionary,
  locales,
  I18nProvider,
  useI18n,
};
