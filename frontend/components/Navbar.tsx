"use client";
import { motion } from "framer-motion";
import Link from "next/link";
import { usePathname } from "next/navigation";

const navItems = [
  { name: "Home", href: "/" },
  { name: "Code", href: "/code" },
  { name: "Echo", href: "/echo" },
  { name: "Glance", href: "/glance" },
  { name: "Persona", href: "/persona" },
];

const Navbar = () => {
  const pathname = usePathname();

  return (
    <div className="w-full mt-4 flex justify-center items-center">
      <nav className="w-[40vw] bg-white/10 backdrop-blur-md border border-white/20 rounded-3xl h-14 flex gap-x-12 justify-around items-center text-white shadow-md">
        {navItems.map((e, i) => (
          <motion.div
            key={e.name}
            initial={{ opacity: 0, y: -10 }}
            animate={{ opacity: 1, y: 0 }}
            whileHover={{ scale: 1.1 }}
            transition={{ delay: i * 0.1, duration: 0.4 }}
          >
            <Link href={e.href}>
              <p
                className={`text-lg text-white font-medium transition duration-200 ${
                  pathname === e.href
                    ? "text-white font-bold "
                    : "hover:text-white/80"
                }`}
              >
                {e.name}
              </p>
            </Link>
          </motion.div>
        ))}
      </nav>
      <div className="h-auto w-auto mr-3 text-4xl"></div>
    </div>
  );
};

export default Navbar;
