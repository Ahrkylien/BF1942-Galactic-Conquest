subshader "jud_upstruts_Material0" "StandardMesh/Default"
{
	lighting true;
	lightingSpecular true;
      materialSpecular 0.8 0.8 0.8;
	materialSpecularPower 4.0;
	envmap true;
	materialDiffuse 0.980392 0.592157 0.592157;
	texture "texture/buildings/judicator/jud_floor";
}

subshader "jud_upstruts_Material1" "StandardMesh/Default"
{
	lighting true;
	lightingSpecular false;
	materialDiffuse 1.0 0.0862745 0.882353;
	texture "texture/buildings/judicator/jud_floornospec";
}

subshader "jud_upstruts_Material2" "StandardMesh/Default"
{
	lighting true;
	lightingSpecular false;
	materialDiffuse 0.984314 0.164706 0.164706;
	texture "texture/buildings/judicator/jud_doorinside";
}

subshader "jud_upstruts_Material3" "StandardMesh/Default"
{
	lighting true;
	lightingSpecular false;
	materialDiffuse 0.588235 0.588235 0.796079;
	texture "texture/buildings/judicator/jud_pipe";
}

subshader "jud_upstruts_Material4" "StandardMesh/Default"
{
	lighting true;
	lightingSpecular false;
	materialDiffuse 0.72549 0.6 0.607843;
	texture "texture/buildings/judicator/jud_pipes";
}

