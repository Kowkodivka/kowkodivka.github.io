import { Component, JSX } from "solid-js";
import { Social } from "@/types";
import RenderIcon from "@/components/ui/RenderIcon";

interface SocialsProps extends JSX.HTMLAttributes<HTMLDivElement> {
  socials: Social[];
}

const Socials: Component<SocialsProps> = (props) => {
  return (
    <div {...props}>
      {props.socials.map(({ icon, link }) => (
        <a
          href={link}
          target="_blank"
          rel="noopener noreferrer"
          class="btn btn-circle"
        >
          <RenderIcon icon={icon} class="size-5" />
        </a>
      ))}
    </div>
  );
};

export default Socials;
