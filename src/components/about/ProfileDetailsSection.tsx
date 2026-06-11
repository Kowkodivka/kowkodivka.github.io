import { Code, Database, Dock, Info, Wrench, Zap } from "lucide-solid";
import { useI18n } from "@/providers/I18nProvider";
import { TechStack } from "@/types";
import RenderIcon from "@/components/ui/RenderIcon";
import {
  siBun,
  siDaisyui,
  siDeno,
  siDocker,
  siFastapi,
  siGit,
  siJavascript,
  siJetbrains,
  siLinux,
  siNumpy,
  siOpencv,
  siPython,
  siPytorch,
  siReact,
  siRust,
  siSolid,
  siSqlite,
  siTailwindcss,
  siTauri,
  siTokio,
  siTypescript,
  siUv,
  siVscodium,
} from "simple-icons";

const techStacks: TechStack[] = [
  {
    icon: siPython,
    name: "Python",
    tools: [
      { icon: siUv, name: "uv", link: "https://docs.astral.sh/uv" },
      {
        icon: siFastapi,
        name: "FastAPI",
        link: "https://fastapi.tiangolo.com/",
      },
      { icon: siPytorch, name: "PyTorch", link: "https://pytorch.org/" },
      { icon: siOpencv, name: "OpenCV", link: "https://opencv.org/" },
      { name: "OpenSlide", link: "https://openslide.org/" },
      { icon: siNumpy, name: "NumPy", link: "https://numpy.org/" },
    ],
  },
  {
    icon: siRust,
    name: "Rust",
    tools: [
      {
        icon: siTokio,
        name: "Tokio",
        link: "https://github.com/tokio-rs/tokio",
      },
      {
        icon: siTokio,
        name: "Axum",
        link: "https://github.com/tokio-rs/axum/",
      },
      {
        name: "teloxide",
        link: "https://github.com/teloxide/teloxide/",
      },
      {
        name: "serenity",
        link: "https://github.com/serenity-rs/serenity/",
      },
      { icon: siTauri, name: "Tauri", link: "https://v2.tauri.app/" },
      { name: "SeaORM", link: "https://www.sea-ql.org/SeaORM/" },
    ],
  },
  {
    icon: Dock,
    name: "Web",
    tools: [
      {
        icon: siJavascript,
        name: "JavaScript",
        link: "https://www.ecma-international.org/",
      },
      {
        icon: siTypescript,
        name: "TypeScript",
        link: "https://www.typescriptlang.org/",
      },
      { icon: siDeno, name: "Deno", link: "https://deno.com/" },
      { icon: siBun, name: "Bun", link: "https://bun.com/" },
      { icon: siReact, name: "React", link: "https://react.dev/" },
      { icon: siSolid, name: "Solid.js", link: "https://www.solidjs.com/" },
      {
        icon: siTailwindcss,
        name: "Tailwind CSS",
        link: "https://tailwindcss.com/",
      },
      { icon: siDaisyui, name: "daisyUI", link: "https://daisyui.com/" },
    ],
  },
  {
    icon: Wrench,
    name: "DevOps & Tools",
    tools: [
      { icon: siDocker, name: "Docker", link: "https://www.docker.com/" },
      { icon: siGit, name: "Git", link: "https://git-scm.com/" },
      {
        icon: siLinux,
        name: "Linux",
        link: "https://en.wikipedia.org/wiki/Linux",
      },
      { icon: siVscodium, name: "VSCodium", link: "https://vscodium.com/" },
      {
        icon: siJetbrains,
        name: "JetBrains IDEs",
        link: "https://www.jetbrains.com/webstorm/",
      },
    ],
  },
  {
    icon: Database,
    name: "Databases",
    tools: [
      { icon: siSqlite, name: "Sqlite", link: "https://www.sqlite.org/" },
    ],
  },
];

const ProfileDetailsSection = () => {
  const { t } = useI18n();

  return (
    <div class="flex-1">
      <div class="w-full join join-vertical bg-base-100">
        <div class="collapse collapse-arrow join-item border-base-300 border rounded-t-2xl">
          <input type="radio" name="my-accordion-4" checked={true} />

          <div class="flex flex-row items-center collapse-title font-semibold gap-1.5">
            <Info class="size-4" /> {t("details.about.sectionName")}
          </div>

          <div class="collapse-content text-sm">
            <p class="mb-4">{t("details.about.bio")}</p>

            <div class="mb-4">
              <p class="font-medium mb-1.5">
                {t("details.about.interestsTitle")}
              </p>
              <ul class="list-disc list-outside ml-5 space-y-1 text-sm">
                <li>{t("details.about.interests.0")}</li>
                <li>{t("details.about.interests.1")}</li>
                <li>{t("details.about.interests.2")}</li>
              </ul>
            </div>

            <p class="mb-4">{t("details.about.lookingFor")}</p>

            <p class="text-sm">{t("details.about.closing")}</p>
          </div>
        </div>

        <div class="collapse collapse-arrow join-item border-base-300 border rounded-b-2xl">
          <input type="radio" name="my-accordion-4" />

          <div class="flex flex-row items-center collapse-title font-semibold gap-1.5">
            <Zap class="size-4" /> {t("details.techStack.sectionName")}
          </div>

          <div class="flex flex-col collapse-content text-sm gap-2">
            {techStacks.map(({ icon, name, tools }) => (
              <div class="flex flex-col bg-base-200 px-4 py-2 gap-1 rounded-2xl">
                <div class="flex items-center font-semibold text-base gap-1.5">
                  <RenderIcon icon={icon} class="size-4" />
                  <span>{name}</span>
                </div>

                <div class="grid grid-cols-1 sm:grid-cols-2">
                  {tools.map(({ icon, name, link }) => (
                    <a
                      href={link}
                      target="_blank"
                      rel="noopener noreferrer"
                      class="flex items-center gap-1.5 p-2 rounded-md hover:bg-base-300 transition-colors duration-150 group"
                    >
                      <RenderIcon
                        icon={icon ? icon : Code}
                        class="size-4 opacity-70 group-hover:opacity-100 text-base-content"
                      />
                      <span class="text-sm">{name}</span>
                    </a>
                  ))}
                </div>
              </div>
            ))}
          </div>
        </div>
      </div>
    </div>
  );
};

export default ProfileDetailsSection;
