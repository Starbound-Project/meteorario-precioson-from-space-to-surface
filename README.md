![Meteorario Banner](banner.png)

# 🌠 Meteorario — Meteor Impact Simulator

> “Predicting the Sky’s Impact on Earth.”

---

## 🚀 Project Overview  
*Meteorario* is an interactive simulation tool that models the physical effects of a meteor impact on Earth.  
Users can modify parameters such as *meteor size, **velocity, **entry angle, and **explosion altitude* to visualize the outcome.  
The simulation calculates *energy, **crater diameter, and **damage radius* using realistic physics-based models.

---

## 🧠 Technical Features  

*Language:* Rust 🦀  
*Interface:* Built with eframe::egui (GUI)  
*Data Management:* Thread-safe state with Arc<Mutex<T>>

### 🔬 Physics Calculations  
- Kinetic energy (Joules → kiloton TNT → megaton TNT)  
- Crater diameter and damage radius estimation  
- Energy attenuation by entry angle and explosion altitude  

### 🌍 Visualization  
- Top-down island map view  
- Animated meteor trajectory  
- Crater and impact zone rendering  

---

## 🎮 Usage Instructions  

1. *Launch* the application.  
2. In the *left panel*, set meteor parameters:  
   - Diameter, velocity, angle, and altitude  
   - X and Z coordinates  
3. Click *“Simulate Impact.”*  
4. View the animation and calculated results in the *right panel.*

---

## 📊 Simulation Results  

| Parameter | Description | Unit |
|------------|--------------|------|
| *Mass* | Estimated meteor mass | tons |
| *Energy* | Impact energy equivalent | kiloton / megaton TNT |
| *Crater Diameter* | Width of the resulting crater | km |
| *Damage Radius* | Zone of destructive impact | km |

### Historical Comparisons  
- 🪨 *Chicxulub* — The asteroid that caused the dinosaur extinction  
- ☄️ *Chelyabinsk (2013, Russia)* — Atmospheric explosion  
- 💣 *Hiroshima (1945)* — Reference energy comparison  

---

## 🧪 Scientific Foundation  
Meteorario uses real-world *physics equations* and *modeling techniques* inspired by *NASA’s educational simulation tools*.  
The magnitude of each meteor impact is dynamically calculated based on entry angle, altitude, and atmospheric resistance.

---

## 👥 Development Team  

> 👩‍🚀 *All team members are 15 years old.*

| Role | Members | Age | Name and Surname
|------|----------|-----|-------|
| *Team Leader / Presenter* | 1 | 15 | İdil Ece Çelik
| *Developer* | 1 | 15 | Mete Parlak
| *Researchers* | 2 | 15 | Eylül Kızılörs, Elif Ensari
| *Designer* | 1 | 15 | Sedef İspir
| *Total Team Members* | *5* | *Average Age: 15* |

*Competition:* NASA International Space Challenge 2025  
*Goal:* To promote scientific awareness and create an educational, interactive simulation tool.

---

## 🪐 Motto  
> “Where the sky meets Earth — calculated.”
