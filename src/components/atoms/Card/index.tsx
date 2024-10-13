import { ComponentProps, ImgHTMLAttributes } from "react";
import { Button as AriaButton } from "react-aria-components";
import styles from "./style.module.scss";

type CardProps = ComponentProps<typeof AriaButton> & {
  title: string;
  subtext: string;
  image: string;
};

const Card = ({
  className = "",
  style,
  title,
  subtext,
  image,
  ...props
}: CardProps) => {
  return (
    <AriaButton className={`${styles["card"]} ${className}`} {...props}>
      <Card.Image src={image} />
      <h2>{title}</h2>
      <p>{subtext}</p>
    </AriaButton>
  );
};

type CardImageProps = ImgHTMLAttributes<HTMLImageElement>;

Card.Image = ({ ...props }: CardImageProps) => {
  return <img {...props} />;
};

export default Card;
