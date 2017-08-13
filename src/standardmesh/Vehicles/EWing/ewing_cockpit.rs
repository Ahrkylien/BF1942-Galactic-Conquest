subshader "ewing_cockpit_Material0" "StandardMesh/Default"
{
	lighting true;
	lightingSpecular true;
	materialSpecular 0.5 0.5 0.5;
	materialSpecularPower 1.5;
        lightingSpecular true;
	envmap true;
        twosided true;
	materialDiffuse 0.5 0.5 0.5;
	texture "texture/vehicles/ewing/ewingcockpit";
}

subshader "ewing_cockpit_Material1" "StandardMesh/Default"
{
	lighting true;
	lightingSpecular false;
	materialDiffuse 0.588 0.588 0.588;
      twosided true;
	transparent true;
      depthwrite false;
	texture "texture/vehicles/xwing/xwing_glass";
}

