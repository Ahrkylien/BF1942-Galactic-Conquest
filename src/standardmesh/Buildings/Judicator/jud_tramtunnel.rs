subshader "jud_tramtunnel_Material0" "StandardMesh/Default"
{
	lighting true;
	lightingSpecular true;
      materialSpecular 0.8 0.8 0.8;
	materialSpecularPower 4.0;
	envmap true;
	materialDiffuse 0.113726 0.0 0.972549;
	texture "texture/buildings/judicator/jud_wall02";
}

subshader "jud_tramtunnel_Material1" "StandardMesh/Default"
{
	lighting true;
	lightingSpecular false;
	materialDiffuse 1.0 0.0862745 0.882353;
	texture "texture/buildings/judicator/jud_floornospec";
}

subshader "jud_tramtunnel_Material2" "StandardMesh/Default"
{
	lighting true;
	lightingSpecular false;
	materialDiffuse 0.243137 0.258824 0.505882;
	texture "texture/buildings/judicator/jud_door02";
}

