subshader "jud_opendoor01_Material0" "StandardMesh/Default"
{
	lighting true;
	lightingSpecular true;
      materialSpecular 0.8 0.8 0.8;
	materialSpecularPower 4.0;
	envmap true;
	materialDiffuse 0.980392 0.592157 0.592157;
	texture "texture/buildings/judicator/jud_floor";
}

subshader "jud_opendoor01_Material1" "StandardMesh/Default"
{
	lighting true;
	lightingSpecular false;
	materialDiffuse 1.0 0.0862745 0.882353;
	texture "texture/buildings/judicator/jud_floornospec";
}

subshader "jud_opendoor01_Material2" "StandardMesh/Default"
{
	lighting true;
	lightingSpecular false;
	materialDiffuse 0.588235 0.588235 0.796079;
	texture "texture/buildings/judicator/jud_pipe";
}

